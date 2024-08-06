use crate::montecarlo_integrable::MontecarloIntegrable;
use lesson1_sampling::step_pdf::DiscretePdf;
use rand::Rng;

fn exp(x: f64) -> f64 {
    (-1000.0 * (x - 0.5).powi(2)).exp()
}

struct Uniform {}
impl MontecarloIntegrable for Uniform {
    type T = f64;

    fn sample(&self, rng: &mut Rng) -> (Self::T, f64) {
        (rng.next_float(), 1.0)
    }
    fn eval(&self, x: Self::T) -> f64 {
        exp(x)
    }
}

struct Importance {
    pdf: DiscretePdf,
}
impl MontecarloIntegrable for Importance {
    type T = f64;

    fn sample(&self, rng: &mut Rng) -> (Self::T, f64) {
        self.pdf.sample(rng)
    }
    fn eval(&self, x: Self::T) -> f64 {
        exp(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn importance_sampling() {
        // Sample from 0 to 1
        let mut file = File::create("data/importance_montecarlo.csv").unwrap();
        file.write_all(b"N,Uniform,Importance,Bad Importance\n")
            .unwrap();

        for pow in 1..22 {
            let n = (2 as usize).pow(pow) as usize;

            let rng = Rng::new();
            let uni = Uniform {};
            let found_uni = uni.integrate(n, rng);

            let rng = Rng::new();
            let imp = Importance {
                pdf: DiscretePdf::new(0.0, vec![0.45, 0.55, 1.0], vec![0.1, 9.1, 0.1]),
            };
            let found_imp = imp.integrate(n, rng);

            let rng = Rng::new();
            let bad_imp = Importance {
                pdf: DiscretePdf::new(0.0, vec![0.45, 0.55, 1.0], vec![1.1, 0.1, 1.1]),
            };
            let found_bad_imp = bad_imp.integrate(n, rng);

            let ln = format!(
                "{},{:.3},{:.3},{:.3}\n",
                pow, found_uni, found_imp, found_bad_imp
            );
            file.write_all(ln.as_bytes()).unwrap();
        }
    }
}
