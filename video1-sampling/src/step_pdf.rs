use rand::Rng;

#[derive(Clone)]
pub struct DiscretePdf {
    steps: Vec<(f64, f64)>,
    min_x: f64,
    max_x: f64,
}

impl DiscretePdf {
    pub fn new(min_x: f64, steps: Vec<f64>, pdfs: Vec<f64>) -> Self {
        if steps.len() != pdfs.len() {
            panic!(
                "Steps and Pdfs are of different length... ({} vs {})",
                steps.len(),
                pdfs.len()
            )
        }

        let max_x = *steps.last().unwrap();

        let steps: Vec<(f64, f64)> = steps.into_iter().zip(pdfs).collect();
        let mut total_acum = 0.0;
        for (i, (max_range, range_pdf)) in steps.iter().enumerate() {
            let min_range = if i == 0 { 0.0 } else { steps[i - 1].0 };
            let delta = max_range - min_range;
            total_acum += delta * *range_pdf;
        }
        if (1.0 - total_acum).abs() > 1e-6 {
            panic!("Expecting total acum to be 1.0... found {:.6}", total_acum)
        }
        Self {
            steps,
            max_x,
            min_x,
        }
    }

    pub fn inv_cdf(&self, y: f64) -> (f64, f64) {
        let mut count = 0;
        let mut min = 0.0;
        let mut max = 1.0;
        assert!(
            y <= 1.0 && y >= 0.0,
            "expecting y to be within 0 to 1 range... found {:.6}",
            y
        );
        loop {
            count += 1;
            if count > 1000 {
                panic!("Too many iterations");
            }
            let x = (min + max) / 2.0;
            let (found_y, found_pdf) = self.cdf(x);
            let err = (y - found_y).abs();
            if err < 0.0001 {
                return (x, found_pdf);
            }
            if y < found_y {
                max = x;
            } else {
                min = x;
            }
        }
    }

    pub fn sample(&self, rng: &mut Rng) -> (f64, f64) {
        let x = rng.next_float();
        self.inv_cdf(x)
    }

    pub fn pdf(&self, x: f64) -> f64 {
        if x < self.min_x {
            return 0.0;
        } else if x >= self.max_x {
            return 0.0;
        } else {
            for (range_max, range_pdf) in self.steps.iter() {
                if x < *range_max {
                    return *range_pdf;
                }
            }
            unreachable!()
        }
    }

    pub fn cdf(&self, x: f64) -> (f64, f64) {
        let mut cum = 0.0;
        let mut last_x = 0.0;
        for (range_max, range_pdf) in self.steps.iter() {
            if x < *range_max {
                return (cum + (x - last_x) * range_pdf, *range_pdf);
            } else {
                cum += (range_max - last_x) * range_pdf;
                last_x = *range_max;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn pdf() {
        let c = DiscretePdf::new(0.0, vec![0.45, 0.55, 1.0], vec![0.1, 9.1, 0.1]);
        let mut file = File::create("data/pdf.csv").unwrap();
        file.write_all(b"x, pdf\n").unwrap();
        for x in 0..100 {
            let x = x as f64 / 100.0;
            let ln = format!("{:.3},{:.3}\n", x, c.pdf(x));
            file.write_all(ln.as_bytes()).unwrap();
        }
    }

    #[test]
    fn cumulative_pdf() {
        let c = DiscretePdf::new(0.0, vec![0.45, 0.55, 1.0], vec![0.1, 9.1, 0.1]);
        let mut file = File::create("data/cdf.csv").unwrap();
        file.write_all(b"x,CDF,PDF\n").unwrap();
        for x in 0..100 {
            let x = x as f64 / 100.0;
            let (cdf, pdf) = c.cdf(x);
            let ln = format!("{:.3},{:.3},{:.3}\n", x, cdf, pdf);
            file.write_all(ln.as_bytes()).unwrap();
        }
    }

    #[test]
    fn inv_cumulative_pdf() {
        let c = DiscretePdf::new(0.0, vec![0.45, 0.55, 1.0], vec![0.1, 9.1, 0.1]);
        let mut file = File::create("data/inv_cdf.csv").unwrap();
        file.write_all(b"x,CDF-1,PDF\n").unwrap();
        for x in 0..100 {
            let x = x as f64 / 100.0;
            let (cdf, pdf) = c.inv_cdf(x);
            let ln = format!("{:.3},{:.3},{:.3}\n", x, cdf, pdf);
            file.write_all(ln.as_bytes()).unwrap();
        }
    }

    #[test]
    fn sample_pdf() {
        let c = DiscretePdf::new(0.0, vec![0.45, 0.55, 1.0], vec![0.1, 9.1, 0.1]);
        let mut rng = Rng::new();
        let mut file = File::create("data/samples.txt").unwrap();
        for _ in 0..8000 {
            let (x, _pdf) = c.sample(&mut rng);
            let ln = format!("{}\n", x);
            file.write_all(ln.as_bytes()).unwrap();
        }
    }
}
