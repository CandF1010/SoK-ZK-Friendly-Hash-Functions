use dusk_curves::bls12_381::BlsScalar;
use dusk_plonk::prelude::*;
use dusk_safe::Safe;

use super::Arion;
use crate::arion::{CONSTANTS_G, CONSTANTS_H, CONSTANTS_AFFINE, D2_INV, MATRIX, WIDTH};


/// An implementation for the Arion permutation operating on [`Witness`]es.
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

impl<'a> Arion<Witness> for GadgetPermutation<'a> {
    fn add_round_constants(
        &mut self,
        round: usize,
        state: &mut [Witness; WIDTH],
    ) {
        state.iter_mut().enumerate().for_each(|(i, w)| {
                let constant = CONSTANTS_AFFINE[round][i];
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

    fn d_2_inv_s_box(&mut self, value: &mut Witness) {
        let witness_x = value.clone();
        let value_x = self.composer[witness_x];
        let value_y = value_x.pow_vartime(&D2_INV);
        let witness_y = self.composer.append_witness(value_y);
        // y^2
        let constraint = Constraint::new().mult(1).a(witness_y).b(witness_y);
        let witness_y_2 = self.composer.gate_mul(constraint);
        // y^4
        let constraint = Constraint::new().mult(1).a(witness_y_2).b(witness_y_2);
        let witness_y_4 = self.composer.gate_mul(constraint);
        // y^8
        let constraint = Constraint::new().mult(1).a(witness_y_4).b(witness_y_4);
        let witness_y_8 = self.composer.gate_mul(constraint);
        // y^16
        let constraint = Constraint::new().mult(1).a(witness_y_8).b(witness_y_8);
        let witness_y_16 = self.composer.gate_mul(constraint);
        // y^32
        let constraint = Constraint::new().mult(1).a(witness_y_16).b(witness_y_16);
        let witness_y_32 = self.composer.gate_mul(constraint);
        // y^64
        let constraint = Constraint::new().mult(1).a(witness_y_32).b(witness_y_32);
        let witness_y_64 = self.composer.gate_mul(constraint);
        // y^128
        let constraint = Constraint::new().mult(1).a(witness_y_64).b(witness_y_64);
        let witness_y_128 = self.composer.gate_mul(constraint);
        // y^256
        let constraint = Constraint::new().mult(1).a(witness_y_128).b(witness_y_128);
        let witness_y_256 = self.composer.gate_mul(constraint);
        // y^257
        let constraint = Constraint::new().mult(1).a(witness_y_256).b(witness_y).fourth(-BlsScalar::one()).d(witness_x);
        *value = self.composer.gate_mul(constraint);
        self.composer.assert_equal(Composer::ZERO, *value);
        *value = witness_y;
    }

    fn mul_matrix(&mut self, state: &mut [Witness; WIDTH]) {
        let mut result = [Composer::ZERO; WIDTH];

        // Implementation optimized for WIDTH = 4
        for j in 0..WIDTH {
            let constraint = Constraint::new()
                .left(MATRIX[j][0])
                .a(state[0])
                .right(MATRIX[j][1])
                .b(state[1])
                .fourth(MATRIX[j][2])
                .d(state[2]);

            result[j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MATRIX[j][3])
                .a(state[3])
                .right(1)
                .b(result[j]);

            result[j] = self.composer.gate_add(constraint);
        }

        state.copy_from_slice(&result);
    }

    fn affine(&mut self, round: usize, state: &mut [Witness; WIDTH]) {
        let mut result = [Composer::ZERO; WIDTH];

        // Implementation optimized for WIDTH = 4
        let c = CONSTANTS_AFFINE[round];
        for j in 0..WIDTH {
            let constraint = Constraint::new()
                .left(MATRIX[j][0])
                .a(state[0])
                .right(MATRIX[j][1])
                .b(state[1])
                .fourth(MATRIX[j][2])
                .d(state[2]);

            result[j] = self.composer.gate_add(constraint);

            let constraint = Constraint::new()
                .left(MATRIX[j][3])
                .a(state[3])
                .right(1)
                .b(result[j])
                .constant(c[j]);

            result[j] = self.composer.gate_add(constraint);
        }

        state.copy_from_slice(&result);
    }

    fn gtds(&mut self, round: usize, state: &mut [Witness; WIDTH]) {
        let state_cpy = state.clone();
        
        self.d_2_inv_s_box(&mut state[WIDTH - 1]);

        let constraint = Constraint::new().left(1).a(state_cpy[WIDTH - 1]).right(1).b(state[WIDTH - 1]);
        let mut sigma = self.composer.gate_add(constraint);
        for i in (0..(WIDTH - 2)).rev() {
            self.quintic_s_box(&mut state[i]);
            let constraint = Constraint::new().mult(1).a(sigma).b(sigma);
            let sigma_squ = self.composer.gate_mul(constraint);
            let constraint = Constraint::new().left(1).a(sigma_squ).right(CONSTANTS_G[round][i][0]).b(sigma).constant(CONSTANTS_G[round][i][1]);
            let g = self.composer.gate_add(constraint);
            let constraint = Constraint::new().left(1).a(sigma_squ).right(CONSTANTS_H[round][i]).b(sigma);
            let h = self.composer.gate_add(constraint);
            let constraint = Constraint::new().mult(1).a(state[i]).b(g);
            state[i] = self.composer.gate_mul(constraint);
            let constraint = Constraint::new().left(1).a(state[i]).right(1).b(h);
            state[i] = self.composer.gate_add(constraint);
            if i > 0 {
                let constraint = Constraint::new().left(1).a(sigma).right(1).b(state_cpy[i]).fourth(1).d(state[i]);
                sigma = self.composer.gate_add(constraint);
            }
        }
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

    use crate::arion::ScalarPermutation;

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

            // Apply Arion gadget permutation.
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
    fn arion() -> ([BlsScalar; WIDTH], [BlsScalar; WIDTH]) {
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
        let label = b"arion_gadget_tester";

        Compiler::compile::<TestCircuit>(&pp, label)
    }

    #[test]
    fn preimage() -> Result<(), Error> {
        let (prover, verifier) = setup()?;

        let (i, o) = arion();

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
