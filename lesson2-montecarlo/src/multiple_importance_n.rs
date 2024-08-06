// use crate::montecarlo_integrable::MontecarloIntegrable;
// use rand::Rng;
// use lesson1_sampling::step_pdf::DiscretePdf;

// struct ABPdfSingle {
//     funcs: Vec<DiscretePdf>,
//     sampling: DiscretePdf,
// }

// impl MontecarloIntegrable for ABPdfSingle {
//     type T = f64;

//     fn sample(&self, _rng: &mut Rng) -> (Self::T, f64) {
//         unreachable!()
//     }

//     fn eval(&self, x: Self::T) -> f64 {
//         let mut ret = 1.0;
//         // the result is the product of all the functions
//         for f in self.funcs.iter() {
//             ret *= f.pdf(x);
//         }
//         ret
//     }
//     fn integrate(&self, n: usize, mut rng: Rng) -> f64 {
//         let mut ret = 0.0;

//         for _ in 0..n {
//             let (xij, pij) = self.sampling.sample(&mut rng);
//             let fxij = self.eval(xij);

//             ret += fxij / pij;
//         }

//         ret / n as f64
//     }
// }

// struct BalancedPdf {
//     funcs: Vec<DiscretePdf>,
//     sampling: Vec<DiscretePdf>,
//     nsamples: Vec<usize>,
// }

// impl MontecarloIntegrable for BalancedPdf {
//     type T = f64;

//     fn sample(&self, _rng: &mut Rng) -> (Self::T, f64) {
//         unreachable!()
//     }

//     fn eval(&self, x: Self::T) -> f64 {
//         let mut ret = 1.0;
//         // the result is the product of all the functions
//         for f in self.funcs.iter() {
//             ret *= f.pdf(x);
//         }
//         ret
//     }
//     fn integrate(&self, n: usize, mut rng: Rng) -> f64 {
//         let mut ret = 0.0;

//         for _ in 0..n {
//             for i in 0..self.sampling.len() {
//                 // Check how many samples from this distribution
//                 let ni = self.nsamples[i];
//                 // For each sample for this function
//                 for _j in 0..ni {
//                     // Draw a Sample
//                     let (xij, pij) = self.sampling[i].sample(&mut rng);
//                     // Calculate the function
//                     let fxij = self.eval(xij);

//                     // Calculate weight

//                     let mut wj_denom = 0.0;
//                     for k in 0..self.sampling.len() {
//                         let pk = self.funcs[k].pdf(xij);
//                         let nk = self.nsamples[k];
//                         wj_denom += nk as f64 * pk;
//                     }
//                     let wij = ni as f64 * pij / wj_denom;
//                     // Add to montecarlo estimator
//                     ret += wij * fxij / (pij * ni as f64);
//                 }
//             }
//         }

//         ret / n as f64
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn multiple_importance_balanced() {
//         let fa = DiscretePdf::new(0.0, vec![0.5, 1.0], vec![0.1, 1.9]);
//         let fb = DiscretePdf::new(0.0, vec![0.5, 1.0], vec![1.9, 0.1]);

//         println!("N,Uniform,A,B,MIS");
//         for pow in 1..16 {
//             let n = (2 as usize).pow(pow) as usize;

//             let rng = Rng::new();
//             let uni = BalancedPdf {
//                 funcs: vec![fa.clone(), fb.clone()],
//                 sampling: vec![DiscretePdf::new(0.0, vec![1.0], vec![1.0])],
//                 nsamples: vec![1],
//             };
//             let found_uni = uni.integrate(n, rng);

//             let rng = Rng::new();
//             let a = BalancedPdf {
//                 funcs: vec![fa.clone(), fb.clone()],
//                 sampling: vec![fa.clone()],
//                 nsamples: vec![1],
//             };
//             let found_a = a.integrate(n, rng);

//             let rng = Rng::new();
//             let b = BalancedPdf {
//                 funcs: vec![fa.clone(), fb.clone()],
//                 sampling: vec![fb.clone()],
//                 nsamples: vec![1],
//             };
//             let found_b = b.integrate(n, rng);

//             let rng = Rng::new();
//             let mis = BalancedPdf {
//                 funcs: vec![fa.clone(), fb.clone()],
//                 sampling: vec![fa.clone(), fb.clone()],
//                 nsamples: vec![1, 1],
//             };
//             let found_mis = mis.integrate(n, rng);

//             println!(
//                 "{},{:.3},{:.3},{:.3},{:.3}",
//                 pow, found_uni, found_a, found_b, found_mis
//             );
//         }
//     }
// }
