use std::default::Default;
use std::ops::Add;
use num::{Float, NumCast};
use noise::Point2;

type NoiseFunc<T> = Fn(usize, &Point2<T>) -> T;

#[derive(Copy, Clone, Debug)]
pub enum BlendMode {
    Add,
    Sub,
    Mul,
}

// 2d noise field
pub struct NoiseField<T: Float + NumCast + Add<T> + Default> {
    pub seed: usize,
    noise: Vec<(BlendMode, Box<NoiseFunc<T>>)>
}

impl<T: Float + NumCast + Add<T> + Default>  NoiseField<T> {
    pub fn new(seed: usize) -> NoiseField<T> {
        NoiseField {
            seed: seed,
            noise: Vec::new(),
        }
    }

    pub fn add_noise(&mut self, func: Box<NoiseFunc<T>>, mode: BlendMode) {
        self.noise.push((mode, func));
    }

    pub fn sample(&self, pt: &[T; 2]) -> T {
        let mut res = Default::default();
        for tuple in self.noise.iter() {
            let mode = tuple.0;
            let f = &tuple.1;
            let n = (f)(self.seed, pt);

            res = match mode {
                BlendMode::Add => res + n,
                BlendMode::Mul => res * n,
                BlendMode::Sub => res - n,
            };
        }
        res
    }
}
