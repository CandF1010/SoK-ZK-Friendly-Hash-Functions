use dusk_curves::bls12_381::BlsScalar;
use dusk_plonk::prelude::*;
use dusk_safe::Safe;

use super::GMiMC2;
use crate::gmimc2::{CONSTANTS, WIDTH};


/// An implementation for the GMiMC2 permutation operating on [`Witness`]es.
/// Requires a reference to a plonk circuit [`Composer`].
pub(crate) struct GadgetPermutation<'a> {
    /// A reference to the constraint system used by the gadgets
    composer: &'a mut Composer,
}

impl<'a> GadgetPermutation<'a> {
    /// Constructs a new `GadgetPermutation` with the constraint system.
    pub fn new(composer: &'a mut Composer) -> Self {
        Self { composer }
    }
}

impl<'a> Safe<Witness, WIDTH> for GadgetPermutation<'a> {
    fn permute(&mut self, state: &mut [Witness; WIDTH]) {
        self.perm(state);
    }

    fn tag(&mut self, input: &[u8]) -> Witness {
        let tag = BlsScalar::hash_to_scalar(input);
        // append the tag as a constant
        self.composer.append_constant(tag)
    }

    fn add(&mut self, right: &Witness, left: &Witness) -> Witness {
        let constraint = Constraint::new().left(1).a(*left).right(1).b(*right);
        self.composer.gate_add(constraint)
    }
}

impl<'a> GMiMC2<Witness> for GadgetPermutation<'a> {
    fn octic_s_box(&mut self, value: &Witness) -> Witness {
        // x^2
        let constraint = Constraint::new().mult(1).a(*value).b(*value);
        let value2 = self.composer.gate_mul(constraint);
        // x^4
        let constraint = Constraint::new().mult(1).a(value2).b(value2);
        let value4 = self.composer.gate_mul(constraint);
        // x^5
        let constraint = Constraint::new().mult(1).a(value4).b(value4);
        
        self.composer.gate_mul(constraint)
    }

    fn first_round_function(&mut self, state: &mut [Witness; WIDTH]) {
        let constraint = Constraint::new().left(1).a(state[0]).constant(CONSTANTS[0]);
        let sum = self.composer.gate_add(constraint);
        let mut result = [Composer::ZERO; WIDTH];
        let pow = self.octic_s_box(&sum);
        let constraint = Constraint::new().left(1).a(state[1]).right(1).b(pow).constant(CONSTANTS[1]);
        result[0] = self.composer.gate_add(constraint);
        for i in (1..(WIDTH - 1)).into_iter() {
            let constraint = Constraint::new().left(1).a(state[i + 1]).right(1).b(pow);
            result[i] = self.composer.gate_add(constraint);
        }
        result[WIDTH - 1] = sum;
        state.copy_from_slice(&result);
    }

    fn round_function(&mut self, round: usize, state: &mut [Witness; WIDTH]) {
        let mut result = [Composer::ZERO; WIDTH];
        let pow = self.octic_s_box(&state[0]);
        let constraint = Constraint::new().left(1).a(state[1]).right(1).b(pow).constant(CONSTANTS[round + 1]);
        result[0] = self.composer.gate_add(constraint);
        for i in (1..(WIDTH - 1)).into_iter() {
            let constraint = Constraint::new().left(1).a(state[i + 1]).right(1).b(pow);
            result[i] = self.composer.gate_add(constraint);
        }
        result[WIDTH - 1] = state[0];
        state.copy_from_slice(&result);
    }

    fn final_round_function(&mut self, state: &mut [Witness; WIDTH]) {
        let mut result = [Composer::ZERO; WIDTH];
        let pow = self.octic_s_box(&state[0]);
        let constraint = Constraint::new().left(1).a(state[1]).right(1).b(pow);
        result[0] = self.composer.gate_add(constraint);
        for i in (0..(WIDTH - 1)).into_iter() {
            let constraint = Constraint::new().left(1).a(state[i + 1]).right(1).b(pow);
            result[i] = self.composer.gate_add(constraint);
        }
        result[WIDTH - 1] = state[0];
        state.copy_from_slice(&result);
    }
}

#[cfg(feature = "encryption")]
impl dusk_safe::Encryption<Witness, WIDTH> for GadgetPermutation<'_> {
    fn subtract(&mut self, minuend: &Witness, subtrahend: &Witness) -> Witness {
        let constraint = Constraint::new()
            .left(1)
            .a(*minuend)
            .right(-BlsScalar::one())
            .b(*subtrahend);
        self.composer.gate_add(constraint)
    }

    fn is_equal(&mut self, lhs: &Witness, rhs: &Witness) -> bool {
        self.composer.assert_equal(*lhs, *rhs);
        // for the encryption to work we need to return true here, the proof
        // creation will fail at a later point if the above assertion isn't met
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::gmimc2::ScalarPermutation;

    use core::result::Result;
    use ff::Field;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[derive(Default)]
    struct TestCircuit {
        i: [BlsScalar; WIDTH],
        o: [BlsScalar; WIDTH],
    }

    impl Circuit for TestCircuit {
        fn circuit(&self, composer: &mut Composer) -> Result<(), Error> {
            let zero = Composer::ZERO;

            let mut perm: [Witness; WIDTH] = [zero; WIDTH];

            let mut i_wit: [Witness; WIDTH] = [zero; WIDTH];
            self.i.iter().zip(i_wit.iter_mut()).for_each(|(i, w)| {
                *w = composer.append_witness(*i);
            });

            let mut o_wit: [Witness; WIDTH] = [zero; WIDTH];
            self.o.iter().zip(o_wit.iter_mut()).for_each(|(o, w)| {
                *w = composer.append_witness(*o);
            });

            // Apply GMiMC2 gadget permutation.
            GadgetPermutation::new(composer).permute(&mut i_wit);

            // Copy the result of the permutation into the perm.
            perm.copy_from_slice(&i_wit);

            // Check that the Gadget perm results = BlsScalar perm results
            i_wit.iter().zip(o_wit.iter()).for_each(|(p, o)| {
                composer.assert_equal(*p, *o);
            });

            Ok(())
        }
    }

    /// Generate a random input and perform a permutation
    fn gmimc2() -> ([BlsScalar; WIDTH], [BlsScalar; WIDTH]) {
        let mut input = [BlsScalar::zero(); WIDTH];

        let mut rng = StdRng::seed_from_u64(0xbeef);

        input
            .iter_mut()
            .for_each(|s| *s = BlsScalar::random(&mut rng));

        let mut output = [BlsScalar::zero(); WIDTH];

        output.copy_from_slice(&input);
        ScalarPermutation::new().permute(&mut output);

        (input, output)
    }

    /// Setup the test circuit prover and verifier
    fn setup() -> Result<(Prover, Verifier), Error> {
        const CAPACITY: usize = 1 << 11;

        let mut rng = StdRng::seed_from_u64(0xbeef);

        let pp = PublicParameters::setup(CAPACITY, &mut rng)?;
        let label = b"gmimc2_gadget_tester";

        Compiler::compile::<TestCircuit>(&pp, label)
    }

    #[test]
    fn preimage() -> Result<(), Error> {
        let (prover, verifier) = setup()?;

        let (i, o) = gmimc2();

        let circuit = TestCircuit { i, o };
        let mut rng = StdRng::seed_from_u64(0xbeef);

        // Proving
        let (proof, public_inputs) = prover.prove(&mut rng, &circuit)?;

        // Verifying
        verifier.verify(&proof, &public_inputs)?;

        Ok(())
    }

    #[test]
    fn preimage_constant() -> Result<(), Error> {
        let (prover, verifier) = setup()?;

        // Prepare input & output
        let i = [BlsScalar::from(5000u64); WIDTH];
        let mut o = [BlsScalar::from(5000u64); WIDTH];
        ScalarPermutation::new().permute(&mut o);

        let circuit = TestCircuit { i, o };
        let mut rng = StdRng::seed_from_u64(0xbeef);

        // Proving
        let (proof, public_inputs) = prover.prove(&mut rng, &circuit)?;

        // Verifying
        verifier.verify(&proof, &public_inputs)?;

        Ok(())
    }

    #[test]
    fn preimage_fails() -> Result<(), Error> {
        let (prover, _) = setup()?;

        // Generate [31, 0, 0, 0, 0] as real input to the perm but build the
        // proof with [31, 31, 31, 31, 31]. This should fail on verification
        // since the Proof contains incorrect statements.
        let x_scalar = BlsScalar::from(31u64);

        let mut i = [BlsScalar::zero(); WIDTH];
        i[1] = x_scalar;

        let mut o = [BlsScalar::from(31u64); WIDTH];
        ScalarPermutation::new().permute(&mut o);

        let circuit = TestCircuit { i, o };
        let mut rng = StdRng::seed_from_u64(0xbeef);

        // Proving should fail
        assert!(
            prover.prove(&mut rng, &circuit).is_err(),
            "proving should fail since the circuit is invalid"
        );

        Ok(())
    }
}
