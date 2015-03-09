use std::default::Default;
use std::ops::Add;
use std::num::{Float, NumCast};
use noise::{Seed, Point2};

// 2d noise field
pub struct NoiseField<T: Float + NumCast + Add<T> + Default> {
    pub seed: Seed,
    noise: Vec<Box<Fn(&Seed, &Point2<T>) -> T>>
}
impl<T: Float + NumCast + Add<T> + Default>  NoiseField<T> {
    pub fn new(seed: Seed) -> NoiseField<T> {
        NoiseField {
            seed: seed,
            noise: Vec::new(),
        }
    }
    pub fn add_noise(&mut self, func: Box<Fn(&Seed, &Point2<T>) -> T>) {
        self.noise.push(func);
    }
    pub fn sample(&self, pt: &[T; 2]) -> T {
        let mut res = Default::default();
        for f in self.noise.iter() {
            res = res + (f)(&self.seed, pt)
        }
        res
    }
}
