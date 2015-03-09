#![feature(core)]
extern crate rand;
extern crate noise;
extern crate rustbox;

use std::default::Default;

use noise::{perlin2, Seed};
use rustbox::{RustBox, Event};
use rustbox::{Style, Color};

mod noisefield;
use noisefield::NoiseField;

type Cell = &'static str;

const CASES: [Cell; 16] = [
    "   ",
    "  _",
    "_  ",
    "___",
    " \\_",
    " | ",
    "/ _",
    "_/ ",
    "_/ ",
    "_ \\",
    " | ",
    "  \\",
    "___",
    "_  ",
    "  _",
    "###",
];

fn corners(x: f32, y: f32, width: f32) -> [[f32; 2]; 4] {
    let w = width / 2.0;
    [[x - w, y - w], [x + w, y - w],
     [x - w, y + w], [x + w, y + w]]
}

fn march(samples: &[f32; 4]) -> Cell {
    let bits: Vec<usize> = samples.iter().map(|&s| if s > 0.2 { 1 } else { 0 }).collect();
    let case = bits[0] << 3 | bits[1] << 2 | bits[2] << 1 | bits[3];
    CASES[case]
}

fn main() {
    let rb = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    const ROWS: usize = 25;
    const COLS: usize = 25;
    let field: NoiseField<f32> = NoiseField::new(Seed::new(0), Box::new(perlin2));

    let mut running = true;
    let mut startx = 0.0f32;
    let mut starty = 0.0f32;
    let mut step = 0.1f32;

    let mut x = startx;
    let mut y = starty;
    
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

                rb.print(ox * 3, oy, rustbox::RB_NORMAL, Color::White, Color::Black, march(&samples));
                x += step;
            }
            y += step;
            x = startx;
        }
        y = starty;

        rb.present();

        match rb.poll_event().unwrap() {
            Event::KeyEvent(_, key, ch) => {
                let k = match key {
                    27 => running = false,      // esc
                    65514 => startx += step,    // right
                    65515 => startx -= step,    // left
                    65516 => starty += step,    // down
                    65517 => starty -= step,    // up
                    0 => {                      // other
                        match ch as u8 as char {
                            'q' => running = false,
                            '+' => step -= 0.01,
                            '-' => step += 0.01,
                            _ => {}
                        }
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }
}
