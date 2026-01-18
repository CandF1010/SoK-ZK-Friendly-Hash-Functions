use super::skyscraper_params::{is_square_round, ExtElem, SkyscraperParams};
use crate::fields::{PrimeField, PrimeFieldExt};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Skyscraper<F: PrimeField + PrimeFieldExt> {
    pub(crate) params: Arc<SkyscraperParams<F>>,
}

impl<F: PrimeField + PrimeFieldExt> Skyscraper<F> {
    pub fn new(params: &Arc<SkyscraperParams<F>>) -> Self {
        Skyscraper {
            params: Arc::clone(params),
        }
    }

    pub fn get_n(&self) -> usize {
        self.params.n
    }

    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let n = self.params.n;
        assert_eq!(input.len(), 2 * n);

        let mut x_l = ExtElem::from_coeffs(input[..n].to_vec());
        let mut x_r = ExtElem::from_coeffs(input[n..].to_vec());

        for round in 0..self.params.rounds {
            let y_l = x_l.clone();
            let mut f = if is_square_round(round) {
                let mut out = x_l.square(&self.params.beta_f);
                if self.params.montgomery {
                    out.mul_scalar(&self.params.sigma_inv);
                }
                out
            } else {
                self.params.bar.apply(&x_l)
            };
            f.add_assign(&self.params.rcons[round]);

            let mut y_r = x_r.clone();
            y_r.add_assign(&f);

            x_l = y_r;
            x_r = y_l;
        }

        let mut out = Vec::with_capacity(2 * n);
        out.extend_from_slice(&x_l.coeffs);
        out.extend_from_slice(&x_r.coeffs);
        out
    }

    pub fn compression(&self, input: &[F]) -> Vec<F> {
        let n = self.params.n;
        assert_eq!(input.len(), 2 * n);
        let mut x_l = ExtElem::from_coeffs(input[..n].to_vec());
        let perm_out = self.permutation(input);
        let mut y_l = ExtElem::from_coeffs(perm_out[..n].to_vec());
        x_l.add_assign(&y_l);
        x_l.coeffs
    }
}
