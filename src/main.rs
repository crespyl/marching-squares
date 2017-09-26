#![feature(slice_patterns)]
extern crate noise;
extern crate rustbox;
extern crate num;

use std::f32::consts::PI;
use std::default::Default;

use noise::NoiseModule;
use rustbox::{RustBox, Event, Key, Color};

mod noisefield;
use noisefield::{Mode, NoiseField};

type Cell = &'static str;

const CASES: [Cell; 16] = [
    r"   ",
    r"_  ",
    r"  _",
    r"___",
    r" \_",
    r"/ _",
    r" | ",
    r"_/ ",
    r"_/ ",
    r" | ",
    r"_ \",
    r"  \",
    r"___",
    r"  _",
    r"_  ",
    r"###",
];

const UNICODE_CASES: [Cell; 16] = [
    r"   ",
    r"─╮ ",
    r" ╭─",
    r"───",
    r" ╰─",
    r"╱▞╱",
    r" │█",
    r"╱▟█",
    r"─╯ ",
    r"█│ ",
    r"╲▚╲",
    r"█▙╲",
    r"───",
    r"█▛╱",
    r"╲▜█",
    r"███",
];

fn distance_kernel(d: f32, mean: f32, sigma: f32) -> f32 {
    let g = 1.0 / (PI/2.0).sqrt();
    let d = (d - mean) / sigma;
    (g * (-0.5 * d * d) / sigma) + 1.0
}

fn corners(x: f32, y: f32, width: f32) -> [[f32; 2]; 4] {
    let w = width / 2.0;
    [[x - w, y - w], [x + w, y - w], [x + w, y + w], [x - w, y + w]]
}

fn march(samples: &[f32; 4], threshold: f32, unicode: bool) -> Cell {
    let bits = [if samples[0] > threshold { 1 } else { 0 },
                if samples[1] > threshold { 1 } else { 0 },
                if samples[2] > threshold { 1 } else { 0 },
                if samples[3] > threshold { 1 } else { 0 }];
    let case = bits[0] << 3 | bits[1] << 2 | bits[2] << 1 | bits[3];
    if unicode { UNICODE_CASES[case] } else { CASES[case] }
}

fn main() {
    let rb = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    // set up noisefield
    let mut field: NoiseField<f32> = NoiseField::new();

    // use the noise crate to set up a perlin+worley noise generator
    let noise_fn = noise::Add::new(noise::Perlin::new(), noise::Worley::new());

    // create a boxed closure that will take ownership of the generator
    field.add_noise(Box::new(move |&[x,y]| noise_fn.get([x,y])), Mode::Add);

    // create a second function that will effectively limit the output field to
    // a circle around the origin
    field.add_noise(Box::new(|&[x, y]| {
        let d = (x*x) + (y*y);
        if d > 5.0 {
            -1.0
        } else {
            1.0
        }
    }), Mode::Add);

    let mut running = true;
    let mut unicode = false;
    let mut step = 0.1f32;
    let mut threshold = 0.4;
    let (mut startx, mut starty) = (0.0f32, 0.0f32);
    let (mut cx, mut cy) = (5.0f32, 0.0f32);

    while running {
        let (rows, cols) = (rb.height(), rb.width() / 3);
        let (mut x, mut y) = (startx - cols as f32 * step / 2.0,
                              starty - rows as f32 * step / 2.0);

        for oy in 0..rows {
            for ox in 0..cols {
                let points = corners(x, y, step);
                let mut samples: [f32; 4] = [field.sample(&points[0]),
                                             field.sample(&points[1]),
                                             field.sample(&points[2]),
                                             field.sample(&points[3])];

                // todo: move interactive circle sampling out into a 'noise' function
                for (sample, c_sample) in samples
                    .iter_mut()
                    .zip(
                        points.iter()
                            .map(|&[sx, sy]| {
                                let (sx, sy) = (sx * 1.5, sy);
                                let d = ((sx-cx)*(sx-cx) + (sy-cy)*(sy-cy)).sqrt();
                                let k = 1.0 / distance_kernel(d, 0.0, 1.0);
                                if k > 0.3 { k*-2.0 } else { 0.0 }
                            }))
                {
                    *sample = *sample + c_sample;
                }

                rb.print(ox * 3, oy,
                         rustbox::RB_NORMAL, Color::White, Color::Black,
                         march(&samples, threshold, unicode));
                x += step;
            }
            y += step;
            x = startx - cols as f32 * step / 2.0;
        }

        rb.present();

        if let Ok(Event::KeyEvent(key)) = rb.poll_event(false) {
            match key {
                Key::Char('w') | Key::Up    => starty -= step,
                Key::Char('s') | Key::Down  => starty += step,
                Key::Char('a') | Key::Left  => startx -= step,
                Key::Char('d') | Key::Right => startx += step,

                Key::Char('i') => cy -= step / 2.0,
                Key::Char('k') => cy += step / 2.0,
                Key::Char('j') => cx -= step / 2.0,
                Key::Char('l') => cx += step / 2.0,

                Key::Char('+') if step > 0.01 => step -= 0.001,
                Key::Char('-') => step += 0.001,
                Key::Char('[') => threshold -= 0.01,
                Key::Char(']') => threshold += 0.01,

                Key::Char('u') => unicode = !unicode,

                Key::Esc | Key::Char('q') => running = false,

                _ => {}
            }
        }
    }
}
