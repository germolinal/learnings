use rand::Rng;

pub trait MontecarloIntegrable {
    type T;

    /// returns a sampled object, and a PDF
    fn sample(&self, rng: &mut Rng) -> (Self::T, f64);

    /// Evaluates a function
    fn eval(&self, x: Self::T) -> f64;

    /// integrates
    fn integrate(&self, n: usize, mut rng: Rng) -> f64 {
        let mut res = 0.0;
        for _ in 0..n {
            let (x, pdf) = self.sample(&mut rng);
            res += self.eval(x) / pdf;
        }

        res / n as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_montecarlo() {
        struct Triangle {
            side: f64,
        }

        impl MontecarloIntegrable for Triangle {
            type T = f64;

            fn sample(&self, rng: &mut Rng) -> (Self::T, f64) {
                (self.side * rng.next_float(), 1.0 / self.side)
            }
            fn eval(&self, x: Self::T) -> f64 {
                x
            }
        }

        struct Quad {
            a: f64,
            b: f64,
            c: f64,
            range: f64,
        }

        impl Quad {
            fn eval_integrated(&self, x: f64) -> f64 {
                self.a * x.powi(3) / 3.0 + self.b * x.powi(2) / 2.0 + self.c * x
            }
            fn analytically_integrate(&self) -> f64 {
                self.eval_integrated(self.range) - self.eval_integrated(0.0)
            }
        }

        impl MontecarloIntegrable for Quad {
            type T = f64;

            fn sample(&self, rng: &mut Rng) -> (Self::T, f64) {
                (self.range * rng.next_float(), 1.0 / self.range)
            }
            fn eval(&self, x: Self::T) -> f64 {
                self.a * x * x + self.b * x + self.c
            }
        }

        println!("pow,err_triangle,err_quad");
        let tri = Triangle { side: 4.0 };
        let exp_tri = tri.side.powi(2) / 2.0;
        let quad = Quad {
            range: 4.0,
            a: 1.0,
            b: 2.121,
            c: 3.0,
        };
        let exp_quad = quad.analytically_integrate();

        for pow in 0..9 {
            let n = (10 as usize).pow(pow as u32) as usize;
            // Triangle
            let rng = Rng::new();
            let found_tri = tri.integrate(n, rng);
            let err_triangle = (exp_tri - found_tri).abs() / exp_tri;

            // Quad
            let rng = Rng::new();
            let found_quad = quad.integrate(n, rng);
            let err_quad = (exp_quad - found_quad).abs() / exp_quad;

            // report
            println!(
                "{},{:.3},{:.3}.",
                pow,
                err_triangle * 100.0,
                err_quad * 100.0,
            );
        }
    }
}
