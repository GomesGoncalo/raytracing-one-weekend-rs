use rand::prelude::*;

pub fn random_between(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}
