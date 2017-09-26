use std::default::Default;
use std::ops::Add;
use num::{Float, NumCast};
use noise::Point2;

type NoiseFunc<T> = Fn(&Point2<T>) -> T;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Add,
    Sub,
    Mul,
}

// 2d noise field defined as a stack of functions that each map from a sample
// point to some floating point value
pub struct NoiseField<T: Float + NumCast + Add<T> + Default> {
    noise: Vec<(Mode, Box<NoiseFunc<T>>)>
}

impl<T: Float + NumCast + Add<T> + Default>  NoiseField<T> {
    // creates a new empty field
    pub fn new() -> NoiseField<T> {
        NoiseField {
            noise: Vec::new()
        }
    }

    // add a function to the field
    pub fn add_noise(&mut self, func: Box<NoiseFunc<T>>, mode: Mode) {
        self.noise.push((mode, func));
    }

    pub fn sample(&self, pt: &[T; 2]) -> T {
        let mut res = Default::default();
        for &(mode, ref f) in self.noise.iter() {
            let n = f(pt);

            res = match mode {
                Mode::Add => res + n,
                Mode::Mul => res * n,
                Mode::Sub => res - n,
            };
        }
        res
    }
}
