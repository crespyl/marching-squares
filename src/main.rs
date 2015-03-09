#![feature(core)]
extern crate rand;
extern crate noise;
extern crate rustbox;

use std::default::Default;

use noise::{perlin2, cell2_value, Brownian2, Seed};
use rustbox::{RustBox, Event, Key, Color};

mod noisefield;
use noisefield::NoiseField;

type Cell = &'static str;

const CASES: [Cell; 16] = [
    r"   ",
    r"  _",
    r"_  ",
    r"___",
    r" \_",
    r" | ",
    r"/ _",
    r"_/ ",
    r"_/ ",
    r"_ \",
    r" | ",
    r"  \",
    r"___",
    r"_  ",
    r"  _",
    r"###",
];

fn corners(x: f32, y: f32, width: f32) -> [[f32; 2]; 4] {
    let w = width / 2.0;
    [[x - w, y - w], [x + w, y - w],
     [x - w, y + w], [x + w, y + w]]
}

fn march(samples: &[f32; 4], threshold: f32) -> Cell {
    let bits: Vec<usize> = samples.iter().map(|&s| if s > threshold { 1 } else { 0 }).collect();
    let case = bits[0] << 3 | bits[1] << 2 | bits[2] << 1 | bits[3];
    CASES[case]
}

fn main() {
    let rb = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    // set up noisefield
    let mut field: NoiseField<f32> = NoiseField::new(Seed::new(0));
    field.add_noise(Box::new(perlin2));
    field.add_noise(Box::new(|seed, &[x, y]| cell2_value(seed, &[x, y]) / 2.0));
    field.add_noise(Box::new(Brownian2::new(perlin2, 5).wavelength(3.0)));
    
    let mut running = true;
    let mut step = 0.1f32;
    let mut threshold = 0.4;
    let (mut startx, mut starty) = (0.0f32, 0.0f32);
    let (mut x, mut y) = (startx, starty);
    
    while running {
        let rows = rb.height();
        let cols = rb.width() / 3;
        for oy in 0..rows {
            for ox in 0..cols {
                let points = corners(x, y, step);
                let samples = [field.sample(&points[0]),
                               field.sample(&points[1]),
                               field.sample(&points[2]),
                               field.sample(&points[3])];

                rb.print(ox * 3, oy, rustbox::RB_NORMAL, Color::White, Color::Black, march(&samples, threshold));
                x += step;
            }
            y += step;
            x = startx;
        }
        y = starty;

        rb.present();

        if let Ok(Event::KeyEvent(Some(key))) = rb.poll_event(false) {
            match key {
                Key::Up    => starty -= step,
                Key::Down  => starty += step,
                Key::Left  => startx -= step,
                Key::Right => startx += step,

                Key::Char('+') => step -= 0.01,
                Key::Char('-') => step += 0.01,
                Key::Char('[') => threshold -= 0.01,
                Key::Char(']') => threshold += 0.01,

                Key::Esc | Key::Char('q') => running = false,

                _ => {}
            }
        }
    }
}
