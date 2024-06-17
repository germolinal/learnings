use std::{thread, time::Duration};

use rand::Rng;
use video1_sampling::step_pdf::DiscretePdf;

pub struct MIS {
    a: DiscretePdf,
    b: DiscretePdf,
    sampling: DiscretePdf,
    roulette: bool,
}

impl MIS {
    pub fn a(&self, x: f64) -> f64 {
        self.a.pdf(x)
    }
    pub fn b(&self, x: f64) -> f64 {
        thread::sleep(Duration::from_nanos(10));
        self.b.pdf(x)
    }

    pub fn sample(&self, rng: &mut Rng) -> (f64, f64) {
        let ret = self.sampling.sample(rng);
        ret
    }

    pub fn eval(&self, x: f64) -> f64 {
        let ret = self.a(x) * self.b(x);
        ret
    }

    /// integrates
    pub fn integrate(&self, n: usize, mut rng: Rng) -> f64 {
        let mut res = 0.0;
        for _ in 0..n {
            let (x, _pdf) = self.sample(&mut rng);

            let eps = rng.next_float();
            // y=3.2 is the max value of a(x), so q will be between 0 and 1
            // if a(x) = 0; then q=1; if a(x)=3.2, then q = 0.
            let q = if self.roulette {
                let ax = self.a(x);
                1.0 - ax / 3.2
            } else {
                0.0
            };
            let c = 0.0;
            // if q is low (i.e., higher a(x)) there are more
            // chances of calculating.
            if eps > q {
                let fx = self.eval(x);
                res += (fx - q * c) / (1.0 - q);
            } else {
                res += c;
            }
        }

        res / n as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::time::Instant;

    #[test]
    fn integrate_russian_roulette() {
        const EXPECTED: f64 = 0.8448;
        fn error(v: f64) -> f64 {
            (v - EXPECTED).abs() / EXPECTED
        }
        let fa = DiscretePdf::new(
            0.0,
            vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0],
            vec![0.5, 1.4, 3.2, 3.0, 0.5, 0.1, 0.1, 0.5, 0.1, 0.6],
        );
        let fb = DiscretePdf::new(
            0.0,
            vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0],
            vec![2.0, 1.0, 0.8, 0.1, 0.02, 0.04, 0.1, 4.0, 0.0, 1.94],
        );
        let uniform = DiscretePdf::new(0.0, vec![1.0], vec![1.]);
        let q0 = MIS {
            a: fa.clone(),
            b: fb.clone(),
            sampling: uniform.clone(),
            roulette: false,
        };
        let q2 = MIS {
            a: fa.clone(),
            b: fb.clone(),
            sampling: uniform.clone(),
            roulette: true,
        };

        let header = b"N,no-roulette,roulette\n";
        let mut results_file = File::create("data/russian-roulette-results.csv").unwrap();
        let mut time_file = File::create("data/russian-roulette-time.csv").unwrap();
        results_file.write_all(header).unwrap();
        time_file.write_all(header).unwrap();
        for pow in 2..16 {
            let n = (2 as usize).pow(pow) as usize;

            let rng = Rng::new();
            let start = Instant::now();
            let found0 = q0.integrate(n, rng);
            let time0 = start.elapsed().as_millis();

            let rng = Rng::new();
            let start = Instant::now();
            let found2 = q2.integrate(n, rng);
            let time2 = start.elapsed().as_millis();

            let ln = format!("{},{:.3},{:.3}\n", pow, error(found0), error(found2),);
            results_file.write_all(ln.as_bytes()).unwrap();

            let ln = format!("{},{:.3},{:.3}\n", pow, time0, time2);
            time_file.write_all(ln.as_bytes()).unwrap();
        }
    }
}
