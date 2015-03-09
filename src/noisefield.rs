use std::num::{Float, NumCast};
use noise::{Seed, Point2};

// 2d noise field
pub struct NoiseField<T: Float + NumCast> {
    pub seed: Seed,
    noise: Box<Fn(&Seed, &Point2<T>) -> T>,
}
impl<T: Float + NumCast>  NoiseField<T> {
    pub fn new(seed: Seed, noise: Box<Fn(&Seed, &Point2<T>) -> T>) -> NoiseField<T> {
        NoiseField {
            seed: seed,
            noise: noise,
        }
    }
    pub fn sample(&self, pt: &[T; 2]) -> T {
        (self.noise)(&self.seed, pt)
    }
}
