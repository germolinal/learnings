use std::f64::consts::PI;

pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn uniform_sample_horizontal_disc(u: (f64, f64), radius: f64) -> (f64, f64) {
    let (r, theta) = u;

    let r = radius * r.sqrt();
    let theta = 2. * crate::PI * theta;
    let (theta_sin, theta_cos) = theta.sin_cos();

    let local_x = r * theta_sin;
    let local_y = r * theta_cos;
    (local_x, local_y)
}

pub fn sample_cosine_weighted_horizontal_hemisphere(u: (f64, f64)) -> Vector3D {
    let (local_x, local_y) = uniform_sample_horizontal_disc(u, 1.);
    let aux = (local_x * local_x + local_y * local_y).clamp(0., 1.);
    let local_z = (1. - aux).sqrt();
    Vector3D {
        x: local_x,
        y: local_y,
        z: local_z,
    }
}

pub fn sample_uniform_hemisphere(u: (f64, f64)) -> Vector3D {
    let rand1 = u.0;
    let rand2 = u.1;
    let z = rand1;
    let r = (1.0 - rand1 * rand1).sqrt();
    let pie2 = 2.0 * PI * rand2;
    let (pie2_sin, pie2_cos) = pie2.sin_cos();
    let x = pie2_cos * r;
    let y = pie2_sin * r;

    Vector3D { x, y, z }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    const EXPECTED_OVERCAST: f64 = PI * 7. / 9.;
    fn overcast_sky(v: Vector3D) -> f64 {
        (1. + 2. * v.z.abs()) / 3.
    }
    const EXPECTED_UNIFORM: f64 = PI;
    fn uniform_sky(_v: Vector3D) -> f64 {
        1.
    }

    #[test]
    fn uniform() {
        let mut rng = Rng::new();

        let mut s = 0.0;
        let n = 1000;
        for _ in 0..n {
            let u = (rng.next_float(), rng.next_float());
            let v = sample_uniform_hemisphere(u); // create a new upward-looking direction
            let cos_theta = v.z; // cosine of the angle between UP and new_dir
            let pdf = 0.5 / PI; // uniformly distributed, so 1/2*PI
                                // let spectrum = uniform_sky(v);
            let spectrum = overcast_sky(v);

            s += spectrum * cos_theta / (pdf);
        }
        s /= n as f64;
        let exp = EXPECTED_OVERCAST;
        let err = (s - exp).abs() / exp;
        println!("{:.7}", err * 100.);
    }

    #[test]
    fn cosine() {
        let mut rng = Rng::new();

        let mut s = 0.0;
        let n = 1000;
        for _ in 0..n {
            let u = (rng.next_float(), rng.next_float());
            let v = sample_cosine_weighted_horizontal_hemisphere(u); // create a new upward-looking direction
            let cos_theta = v.z; // cosine of the angle between UP and new_dir
            let pdf = cos_theta / PI; // uniformly distributed, so 1/2*PI
                                      // let spectrum = uniform_sky(v);
            let spectrum = overcast_sky(v);

            s += spectrum * cos_theta / pdf;
        }
        s /= n as f64;
        let exp = EXPECTED_OVERCAST;
        let err = (s - exp).abs() / exp;
        println!("{:.7}", err * 100.)
    }
}
