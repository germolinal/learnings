use crate::montecarlo_integrable::MontecarloIntegrable;
use rand::Rng;
use video1_sampling::step_pdf::DiscretePdf;

struct ABPdfSingle {
    a: DiscretePdf,
    b: DiscretePdf,
    sampling: DiscretePdf,
}

impl MontecarloIntegrable for ABPdfSingle {
    type T = f64;

    fn sample(&self, rng: &mut Rng) -> (Self::T, f64) {
        let ret = self.sampling.sample(rng);
        if ret.0.is_nan() || ret.1.is_nan() || ret.1 < 1e-7 {
            dbg!(ret);
        }
        ret
    }

    fn eval(&self, x: Self::T) -> f64 {
        let ret = self.a.pdf(x) * self.b.pdf(x);
        if ret.is_nan() {
            dbg!(ret, x, self.a.pdf(x), self.b.pdf(x));
        }
        ret
    }
}

struct BalancedPdf {
    a: DiscretePdf,
    b: DiscretePdf,
    na: usize,
    nb: usize,
}

impl BalancedPdf {
    fn w(&self, na: usize, pdfa: f64, nb: usize, pdfb: f64) -> f64 {
        let na = na as f64;
        let nb = nb as f64;
        na * pdfa / (nb * pdfb + na * pdfa)
    }
}

impl MontecarloIntegrable for BalancedPdf {
    type T = f64;

    fn sample(&self, _rng: &mut Rng) -> (Self::T, f64) {
        unreachable!()
    }

    fn eval(&self, x: Self::T) -> f64 {
        self.a.pdf(x) * self.b.pdf(x)
    }

    fn integrate(&self, n: usize, mut rng: Rng) -> f64 {
        let mut ret = 0.0;

        for _ in 0..n {
            for _ in 0..self.na {
                let (x, pax) = self.a.sample(&mut rng);
                let pbx = self.b.pdf(x);
                let wa = self.w(self.na, pax, self.nb, pbx);
                let fx = self.eval(x);
                ret += wa * fx / pax / (self.na as f64);
            }

            for _ in 0..self.nb {
                let (y, pby) = self.b.sample(&mut rng);
                let fy = self.eval(y);
                let pay = self.a.pdf(y);
                let wb = self.w(self.nb, pby, self.na, pay);
                ret += wb * fy / pby / (self.nb as f64);
            }
        }

        ret / n as f64
    }
}

struct PowerPdf {
    a: DiscretePdf,
    b: DiscretePdf,
    na: usize,
    nb: usize,
}

impl PowerPdf {
    fn w(&self, na: usize, pdfa: f64, nb: usize, pdfb: f64) -> f64 {
        let na = na as f64;
        let nb = nb as f64;
        let a = na * pdfa;
        let b = nb * pdfb;
        a * a / (a * a + b * b)
    }
}

impl MontecarloIntegrable for PowerPdf {
    type T = f64;

    fn sample(&self, _rng: &mut Rng) -> (Self::T, f64) {
        unreachable!()
    }

    fn eval(&self, x: Self::T) -> f64 {
        self.a.pdf(x) * self.b.pdf(x)
    }

    fn integrate(&self, n: usize, mut rng: Rng) -> f64 {
        let mut ret = 0.0;

        for _ in 0..n {
            for _ in 0..self.na {
                let (x, pax) = self.a.sample(&mut rng);
                let pbx = self.b.pdf(x);
                let wa = self.w(self.na, pax, self.nb, pbx);
                let fx = self.eval(x);
                ret += wa * fx / pax / (self.na as f64);
            }

            for _ in 0..self.nb {
                let (y, pby) = self.b.sample(&mut rng);
                let fy = self.eval(y);
                let pay = self.a.pdf(y);
                let wb = self.w(self.nb, pby, self.na, pay);
                ret += wb * fy / pby / (self.nb as f64);
            }
        }

        ret / n as f64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn multiple_importance() {
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

        let mut file = File::create("data/mis_montecarlo.csv").unwrap();

        file.write_all(b"N,Uniform,A,B,Balanced MIS,Power MIS\n")
            .unwrap();
        for pow in 1..16 {
            let n = (2 as usize).pow(pow) as usize;

            let rng = Rng::new();
            let uni = ABPdfSingle {
                a: fa.clone(),
                b: fb.clone(),
                sampling: uniform.clone(),
            };
            let found_uni = uni.integrate(n, rng);

            let rng = Rng::new();
            let a = ABPdfSingle {
                a: fa.clone(),
                b: fb.clone(),
                sampling: fa.clone(),
            };
            let found_a = a.integrate(n, rng);

            let rng = Rng::new();
            let b = ABPdfSingle {
                a: fa.clone(),
                b: fb.clone(),
                sampling: fb.clone(),
            };
            let found_b = b.integrate(n, rng);

            let rng = Rng::new();
            let mis = BalancedPdf {
                a: fa.clone(),
                b: fb.clone(),
                na: 9,
                nb: 5,
            };
            let found_mis = mis.integrate(n, rng);

            let rng = Rng::new();
            let power_mis = PowerPdf {
                a: fa.clone(),
                b: fb.clone(),
                na: 9,
                nb: 5,
            };
            let found_power_mis = power_mis.integrate(n, rng);

            let ln = format!(
                "{},{:.3},{:.3},{:.3},{:.3},{:.3}\n",
                pow,
                error(found_uni),
                error(found_a),
                error(found_b),
                error(found_mis),
                error(found_power_mis),
            );
            file.write_all(ln.as_bytes()).unwrap();
        }
    }
}
