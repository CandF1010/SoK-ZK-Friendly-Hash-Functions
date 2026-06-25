use dusk_curves::bls12_381::BlsScalar;
use dusk_plonk::prelude::*;
use dusk_safe::Safe;

use super::Anemoi;
use crate::anemoi::{FIVE_INV, G, G_INV, G_1, G_2, G_SQU_1, G_SQU_G_1, G_SQU_2G_1, ROUND_CONSTANTS, WIDTH};


/// An implementation for the Anemoi permutation operating on [`Witness`]es.
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

impl<'a> Anemoi<Witness> for GadgetPermutation<'a> {
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

    fn open_flystel(&mut self, state: &mut [BlsScalar]) {
        let mut feistel_1 = state[1].square();
        feistel_1 *= G;
        feistel_1 += G_INV;
        state[0] -= feistel_1;
        //
        let feistel_2 = state[0].pow_vartime(&FIVE_INV);
        state[1] -= feistel_2; 
        //
        let mut feistel_3 = state[1].square();
        feistel_3 *= G;
        state[0] += feistel_3;
    }

    fn closed_flystel(&mut self, state_y_v: &[Witness], state_x_u: &[Witness]) {
        let constraint = Constraint::new().left(1).a(state_y_v[0]).right(-BlsScalar::one()).b(state_y_v[1]);
        let y_u = self.composer.gate_add(constraint);
        // (y - u)^2
        let constraint = Constraint::new().mult(1).a(y_u).b(y_u);
        let y_u_2 = self.composer.gate_mul(constraint);
        // (y - u)^4
        let constraint = Constraint::new().mult(1).a(y_u_2).b(y_u_2);
        let y_u_4 = self.composer.gate_mul(constraint);
        // (y - u)^5
        let constraint = Constraint::new().mult(1).a(y_u_4).b(y_u);
        let y_u_5 = self.composer.gate_mul(constraint);
        // 
        let constraint = Constraint::new().mult(G).a(state_y_v[0]).b(state_y_v[0]).constant(G_INV);
        let feistel_1 = self.composer.gate_mul(constraint);
        //
        let constraint = Constraint::new().mult(G).a(state_y_v[1]).b(state_y_v[1]);
        let feistel_2 = self.composer.gate_mul(constraint);
        //
        let constraint = Constraint::new().left(1).a(feistel_1).right(1).b(y_u_5);
        let sum_1 = self.composer.gate_add(constraint);
        self.composer.assert_equal(state_x_u[0], sum_1);
        //
        let constraint = Constraint::new().left(1).a(feistel_2).right(1).b(y_u_5);
        let sum_2 = self.composer.gate_add(constraint);
        self.composer.assert_equal(state_x_u[1], sum_2);
    }

    fn linear_layer(&mut self, state: &mut [Witness; WIDTH]) {
        // Anemoi matrix for WITDH = 4 is given by
        // 2 + g,        1 + g,       0,            0
        // 0,            0,           g^2 + 2g + 1, g^2 + g + 1
        // g^2 + 2g + 1, g^2 + g + 1, 0,          , 0
        // 0,            0,         , 2 + g       , 1 + g
        let mut result = [Composer::ZERO; WIDTH];

        let constraint = Constraint::new().left(G_2).a(state[0]).right(G_1).b(state[1]);
        result[0] = self.composer.gate_add(constraint);
        let constraint = Constraint::new().left(G_SQU_2G_1).a(state[2]).right(G_SQU_G_1).b(state[3]);
        result[1] = self.composer.gate_add(constraint);
        let constraint = Constraint::new().left(G_SQU_2G_1).a(state[0]).right(G_SQU_G_1).b(state[1]);
        result[2] = self.composer.gate_add(constraint);
        let constraint = Constraint::new().left(G_2).a(state[2]).right(G_1).b(state[3]);
        result[3] = self.composer.gate_add(constraint);

        state.copy_from_slice(&result);
    }

    fn linear_layer_final(&mut self, state: &mut [Witness; WIDTH]) {
        // M_x
        // 1,   g
        // g,   g^2 + 1
        //
        // M_y
        // g,   g^2 + 1
        // 1,   g
        let mut result = [Composer::ZERO; WIDTH];
        
        let constraint = Constraint::new().left(1).a(state[0]).right(G).b(state[1]);
        result[0] = self.composer.gate_add(constraint);
        let constraint = Constraint::new().left(G).a(state[0]).right(G_SQU_1).b(state[1]);
        result[1] = self.composer.gate_add(constraint);
        let constraint = Constraint::new().left(G).a(state[2]).right(G_SQU_1).b(state[3]);
        result[2] = self.composer.gate_add(constraint);
        let constraint = Constraint::new().left(1).a(state[2]).right(G).b(state[3]);
        result[3] = self.composer.gate_add(constraint);

        state.copy_from_slice(&result);
    }

    fn affine_layer(&mut self, round: usize, state: &mut [Witness; WIDTH]) {
        // Anemoi matrix for WITDH = 4 is given by
        // 2 + g,        1 + g,       0,            0
        // 0,            0,           g^2 + 2g + 1, g^2 + g + 1
        // g^2 + 2g + 1, g^2 + g + 1, 0,          , 0
        // 0,            0,         , 2 + g       , 1 + g
        let mut result = [Composer::ZERO; WIDTH];
        let c = ROUND_CONSTANTS[round];

        let constraint = Constraint::new().left(G_2).a(state[0]).right(G_1).b(state[1]).constant(c[0]);
        result[0] = self.composer.gate_add(constraint);
        let constraint = Constraint::new().left(G_SQU_2G_1).a(state[2]).right(G_SQU_G_1).b(state[3]).constant(c[1]);
        result[1] = self.composer.gate_add(constraint);
        let constraint = Constraint::new().left(G_SQU_2G_1).a(state[0]).right(G_SQU_G_1).b(state[1]).constant(c[2]);
        result[2] = self.composer.gate_add(constraint);
        let constraint = Constraint::new().left(G_2).a(state[2]).right(G_1).b(state[3]).constant(c[3]);
        result[3] = self.composer.gate_add(constraint);

        state.copy_from_slice(&result);
    }

    fn round_function(&mut self, round: usize, state: &mut [Witness; WIDTH]) {
        self.affine_layer(round, state);

        for i in 0..(WIDTH / 2){
            let value_x_y = [
                                 self.composer[state[2 * i]],
                                 self.composer[state[2 * i + 1]],
                                ]; 
            let mut value_u_v = value_x_y.clone();
            self.open_flystel(&mut value_u_v);
            let witness_y_v = [
                                            self.composer.append_witness(value_x_y[1]),
                                            self.composer.append_witness(value_u_v[1]),
                                        ];
            let witness_x_u = [
                                            self.composer.append_witness(value_x_y[0]),
                                            self.composer.append_witness(value_u_v[0]),
                                            ];
            self.closed_flystel(&witness_y_v, &witness_x_u);
            state[2 * i] = self.composer.append_witness(value_u_v[0]);
            state[2 * i + 1] = self.composer.append_witness(value_u_v[1]);
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

    use crate::anemoi::ScalarPermutation;

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

            // Apply Anemoi gadget permutation.
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
    fn anemoi() -> ([BlsScalar; WIDTH], [BlsScalar; WIDTH]) {
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
        let label = b"anemoi_gadget_tester";

        Compiler::compile::<TestCircuit>(&pp, label)
    }

    #[test]
    fn preimage() -> Result<(), Error> {
        let (prover, verifier) = setup()?;

        let (i, o) = anemoi();

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
