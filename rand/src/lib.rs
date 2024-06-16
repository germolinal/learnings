use std::time::{SystemTime, UNIX_EPOCH};

pub struct Rng {
    seed: u64,
}

impl Rng {
    pub fn new() -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let seed = since_the_epoch.as_nanos() as u64;

        Rng { seed }
    }

    fn next(&mut self) -> u64 {
        // Parameters for Rng (these values are chosen to provide a full period)
        const A: u64 = 6364136223846793005;
        const C: u64 = 1;
        const M: u64 = 1 << 32;

        // Update the seed using the Rng formula
        self.seed = (A.wrapping_mul(self.seed).wrapping_add(C)) % M;
        self.seed
    }

    pub fn next_float(&mut self) -> f64 {
        // Convert to a floating point number in the range [0, 1)
        self.next() as f64 / (1u64 << 32) as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::Rng;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn sample() {
        let mut rng = Rng::new();
        let mut file = File::create("uniform_samples.txt").unwrap();
        for _ in 0..10000 {
            let ln = format!("{}\n", rng.next_float());
            file.write_all(ln.as_bytes()).unwrap();
        }
    }
}
