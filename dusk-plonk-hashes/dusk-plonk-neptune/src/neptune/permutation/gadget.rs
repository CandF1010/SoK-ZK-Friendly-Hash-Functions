use dusk_curves::bls12_381::BlsScalar;
use dusk_plonk::prelude::*;
use dusk_safe::Safe;

use crate::neptune::{ALPHA, GAMMA, MATRIX_EXTERNAL_1, MATRIX_EXTERNAL_2, MATRIX_INTERNAL, ROUND_CONSTANTS, WIDTH};

use super::Neptune;

/// An implementation for the [`Neptune`] permutation operating on [`Witness`]es.
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

impl<'a> Neptune<Witness> for GadgetPermutation<'a> {
    fn add_round_constants(
        &mut self,
        round: usize,
        state: &mut [Witness; WIDTH],
    ) {
        state.iter_mut().enumerate().for_each(|(i, w)| {
                let constant = ROUND_CONSTANTS[round][i];
                let constraint =
                    Constraint::new().left(1).a(*w).constant(constant);

                *w = self.composer.gate_add(constraint);
            });
    }

    fn quintic_s_box(&mut self, value: &mut Witness) {
        let constraint = Constraint::new().mult(1).a(*value).b(*value);
        let v2 = self.composer.gate_mul(constraint);

        let constraint = Constraint::new().mult(1).a(v2).b(v2);
        let v4 = self.composer.gate_mul(constraint);

        let constraint = Constraint::new().mult(1).a(v4).b(*value);
        *value = self.composer.gate_mul(constraint);
    }

    fn lm_s_box(&mut self, state: &mut [Witness]) {
        // z_1 = x_0 - x_1
        let constraint = Constraint::new()
            .left(1)
            .a(state[0])
            .right(-BlsScalar::one())
            .b(state[1]);
        let z_1 = self.composer.gate_add(constraint);
        // z_2 = z_1^2
        let constraint = Constraint::new()
            .mult(1)
            .a(z_1)
            .b(z_1);
        let z_2 = self.composer.gate_mul(constraint);
        // z_3 = -z_2 + alpha * (x_0 - 2 * x_1) + gamma
        let constraint = Constraint::new()
            .left(-BlsScalar::one())
            .a(z_2)
            .right(ALPHA)
            .b(z_1)
            .fourth(-ALPHA)
            .d(state[1])
            .constant(GAMMA);
        let z_3 = self.composer.gate_add(constraint);
        // z_4 = z_3^2
        let constraint = Constraint::new()
            .mult(1)
            .a(z_3)
            .b(z_3);
        let z_4 = self.composer.gate_mul(constraint);
        // z_5 = 3 * alpha * z_2 + alpha^2 * (2 * x_0 + x_1)
        let constraint = Constraint::new()
            .left(ALPHA.double() + ALPHA)
            .a(z_2)
            .right(ALPHA.square().double())
            .b(state[0])
            .fourth(ALPHA.square())
            .d(state[1]);
        let z_5 = self.composer.gate_add(constraint);
        // z_6 = 4 * alpha * z_2 + alpha^2 * (x_0 + 3 * x_1)
        let constraint = Constraint::new()
            .left(ALPHA.double().double())
            .a(z_2)
            .right(ALPHA.square())
            .b(state[0])
            .fourth(ALPHA.square().double() + ALPHA.square())
            .d(state[1]);
        let z_6 = self.composer.gate_add(constraint);
        
        let mut result = [Composer::ZERO; 2];
        // y_0 = z_5 + z_4
        let constraint = Constraint::new()
            .left(1)
            .a(z_5)
            .right(1)
            .b(z_4);
        result[0] = self.composer.gate_add(constraint);
        // y_1 = z_6 + z_4
        let constraint = Constraint::new()
            .left(1)
            .a(z_6)
            .right(1)
            .b(z_4);
        result[1] = self.composer.gate_add(constraint);

        state.copy_from_slice(&result);
    }

    fn mul_matrix_external(&mut self, state: &mut [Witness; WIDTH]) {
        let mut result = [Composer::ZERO; WIDTH];

        // Implementation optimized for WIDTH = 4
        for j in 0..(WIDTH / 2) {
            let constraint = Constraint::new()
                .left(MATRIX_EXTERNAL_1[j][0])
                .a(state[0])
                .right(MATRIX_EXTERNAL_1[j][1])
                .b(state[2]);

            result[2 * j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MATRIX_EXTERNAL_2[j][0])
                .a(state[1])
                .right(MATRIX_EXTERNAL_2[j][1])
                .b(state[3]);

            result[2 * j + 1] = self.composer.gate_add(constraint);
        }

        state.copy_from_slice(&result);
    }

    fn affine_external(&mut self, round: usize, state: &mut [Witness; WIDTH]) {
        let mut result = [Composer::ZERO; WIDTH];

        // Implementation optimized for WIDTH = 4
        let c = ROUND_CONSTANTS[round];
        for j in 0..(WIDTH / 2) {
            let constraint = Constraint::new()
                .left(MATRIX_EXTERNAL_1[j][0])
                .a(state[0])
                .right(MATRIX_EXTERNAL_1[j][1])
                .b(state[2])
                .constant(c[2 * j]);

            result[2 * j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MATRIX_EXTERNAL_2[j][0])
                .a(state[1])
                .right(MATRIX_EXTERNAL_2[j][1])
                .b(state[3])
                .constant(c[2 *j + 1]);

            result[2 * j + 1] = self.composer.gate_add(constraint);
        }

        state.copy_from_slice(&result);
    }

    fn affine_internal(&mut self, round: usize, state: &mut [Witness; WIDTH]) {
        let mut result = [Composer::ZERO; WIDTH];

        let c = ROUND_CONSTANTS[round];
        for j in 0..WIDTH {
            let constraint = Constraint::new()
                .left(MATRIX_INTERNAL[j][0])
                .a(state[0])
                .right(MATRIX_INTERNAL[j][1])
                .b(state[1])
                .fourth(MATRIX_INTERNAL[j][2])
                .d(state[2]);

            result[j] = self.composer.gate_add(constraint);
            let constraint = Constraint::new()
                .left(MATRIX_INTERNAL[j][3])
                .a(state[3])
                .right(1)
                .b(result[j])
                .constant(c[j]);

            result[j] = self.composer.gate_add(constraint);
        }

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

    use crate::neptune::ScalarPermutation;

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

            // Apply Neptune gadget permutation.
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
    fn neptune() -> ([BlsScalar; WIDTH], [BlsScalar; WIDTH]) {
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
        const CAPACITY: usize = 1 << 10;

        let mut rng = StdRng::seed_from_u64(0xbeef);

        let pp = PublicParameters::setup(CAPACITY, &mut rng)?;
        let label = b"neptune_gadget_tester";

        Compiler::compile::<TestCircuit>(&pp, label)
    }

    #[test]
    fn preimage() -> Result<(), Error> {
        let (prover, verifier) = setup()?;

        let (i, o) = neptune();

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
