use dusk_curves::bls12_381::BlsScalar;
use dusk_plonk::prelude::*;
use dusk_safe::Safe;

use super::GMiMC;
use crate::gmimc::{CONSTANTS, WIDTH};


/// An implementation for the GMiMC permutation operating on [`Witness`]es.
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

impl<'a> GMiMC<Witness> for GadgetPermutation<'a> {
    fn quintic_s_box(&mut self, round: usize, value: &Witness) -> Witness {
        let constraint = Constraint::new().left(1).a(*value).constant(CONSTANTS[round]);
        let sum = self.composer.gate_add(constraint);

        // x^2
        let constraint = Constraint::new().mult(1).a(sum).b(sum);
        let sum2 = self.composer.gate_mul(constraint);
        // x^4
        let constraint = Constraint::new().mult(1).a(sum2).b(sum2);
        let sum4 = self.composer.gate_mul(constraint);
        // x^5
        let constraint = Constraint::new().mult(1).a(sum4).b(sum);
        
        self.composer.gate_mul(constraint)
    }

    fn round_function(&mut self, round: usize, state: &mut [Witness; WIDTH]) {
        let mut result = [Composer::ZERO; WIDTH];
        let pow = self.quintic_s_box(round, &state[0]);
        for i in (0..(WIDTH - 1)).into_iter() {
            let constraint = Constraint::new().left(1).a(state[i + 1]).right(1).b(pow);
            result[i] = self.composer.gate_add(constraint);
        }
        result[WIDTH - 1] = state[0];
        state.copy_from_slice(&result);
    }

    /*
    // Does not work
    fn perm_opt(&mut self, state: &mut [Witness; WIDTH]) {
        let mut current_state = [Composer::ZERO; WIDTH];
        current_state.copy_from_slice(state);
        let mut acc = Composer::ZERO;
        let mut acc_queue = [Composer::ZERO; WIDTH - 1];
        for round in (0..ROUNDS).into_iter() {
            let pow = self.quintic_s_box(round, &current_state[0]);
            acc_queue.rotate_left(1);
            let constraint = Constraint::new().left(1).a(acc).right(Q_MINUS_1).b(acc_queue[0]);
            acc = self.composer.gate_add(constraint);
            let constraint = Constraint::new().left(1).a(Composer::ZERO).right(1).b(pow);
            acc_queue[0] = self.composer.gate_add(constraint);
            //acc_queue[0].clone_from(&pow);
            let constraint = Constraint::new().left(1).a(acc).right(1).b(pow);
            acc = self.composer.gate_add(constraint);

            current_state.rotate_left(1);
            let constraint = Constraint::new().left(1).a(acc).right(1).b(current_state[0]);
            current_state[0] = self.composer.gate_add(constraint);
        }

        // final adds
        for i in (1..(WIDTH - 1)).into_iter() {
            acc_queue.rotate_left(1);
            let constraint = Constraint::new().left(1).a(acc).right(Q_MINUS_1).b(acc_queue[0]);
            acc = self.composer.gate_add(constraint);
            let constraint = Constraint::new().left(1).a(acc).right(1).b(current_state[i]);
            current_state[i] = self.composer.gate_add(constraint);
        }

        state.copy_from_slice(&current_state);
    }
    */
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

    use crate::gmimc::ScalarPermutation;

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

            // Apply GMiMC gadget permutation.
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
    fn gmimc() -> ([BlsScalar; WIDTH], [BlsScalar; WIDTH]) {
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
        let label = b"gmimc_gadget_tester";

        Compiler::compile::<TestCircuit>(&pp, label)
    }

    #[test]
    fn preimage() -> Result<(), Error> {
        let (prover, verifier) = setup()?;

        let (i, o) = gmimc();

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
