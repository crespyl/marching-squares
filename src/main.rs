#![feature(slice_patterns)]
extern crate noise;
extern crate rustbox;

use std::default::Default;

use noise::NoiseFn;
use rustbox::{RustBox, Event, Key, Color};

// Cells are sampled sampled clockwise, like so:
//
// 12
// 43
//
const CASES: [[&'static str; 2]; 16] = [
    // 00
    // 00
    ["  ",
     "  "],

    // 00
    // 10
    [r"  ",
     r". ",],

    // 00
    // 01
    ["  ",
     " ."],

    // 00
    // 11
    ["  ",
     "--"],

    // 01
    // 00
    [r" .",
     r"  "],

    // 01
    // 10
    [r" .",
     r". "],

    // 01
    // 01
    [" |",
     " |"],

    // 01
    // 11
    [" /",
     "/#"],

    // 10
    // 00
    [". ",
     "  "],

    // 10
    // 10
    ["| ",
     "| "],

    // 10
    // 01
    [". ",
     " ."],

    // 10
    // 11
    [r"\ ",
     r"#\"],

    // 11
    // 00
    ["--",
     "  "],

    // 11
    // 10
    ["#/",
     "/ "],

    // 11
    // 01
    [r"\#",
     r" \"],

    // 11
    // 11
    ["##",
     "##"],
];

const CASES_UNICODE: [[&'static str; 2]; 16] = [
    // 00
    // 00
    ["  ",
     "  "],

    // 00
    // 10
    [r"  ",
     r"╮ ",],

    // 00
    // 01
    ["  ",
     " ╭"],

    // 00
    // 11
    ["  ",
     "──"],

    // 01
    // 00
    [r" ╰",
     r"  "],

    // 01
    // 10
    [r" ╰",
     r"╮ "],

    // 01
    // 01
    [" │",
     " │"],

    // 01
    // 11
    ["╭╯",
     "╯#"],

    // 10
    // 00
    ["╯ ",
     "  "],

    // 10
    // 10
    ["│ ",
     "│ "],

    // 10
    // 01
    ["╯ ",
     " ╭"],

    // 10
    // 11
    [r"╰╮",
     r"#╰"],

    // 11
    // 00
    ["──",
     "  "],

    // 11
    // 10
    ["#╭",
     "╭╯"],

    // 11
    // 01
    [r"╮#",
     r"╰╮"],

    // 11
    // 11
    ["##",
     "##"],
];

fn corners(x: f64, y: f64, width: f64) -> [[f64; 2]; 4] {
    let w = width / 2.0;
    [[x - w, y - w], [x + w, y - w], [x + w, y + w], [x - w, y + w]]
}

fn samples_to_idx(samples: &[f64; 4], threshold: f64) -> usize {
    let bits = [if samples[0] > threshold { 1 } else { 0 },
                if samples[1] > threshold { 1 } else { 0 },
                if samples[2] > threshold { 1 } else { 0 },
                if samples[3] > threshold { 1 } else { 0 }];
    bits[0] << 3 | bits[1] << 2 | bits[2] << 1 | bits[3]
}

fn main() {
    let rb = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    // use the noise crate to set up a combination noise generator

    let billow = noise::Billow::new();

    let checkerboard = noise::Checkerboard::new();
    let scale_point = noise::ScalePoint::new(
        &checkerboard
    ).set_x_scale(0.1).set_y_scale(0.1);
    let turbulence = noise::Turbulence::new(&scale_point).set_power(0.3);

    let worley = noise::Worley::new();

    let noise_fn = noise::Blend::new(&billow, &turbulence, &worley);

    let mut running = true;
    let mut unicode = false;
    let mut step = 0.1f64;
    let mut threshold = 0.25;
    let (mut startx, mut starty) = (0.0f64, 0.0f64);

    while running {
        let (rows, cols) = (rb.height() / 2, rb.width() / 2);
        let (mut x, mut y) = (startx - cols as f64 * step / 2.0,
                              starty - rows as f64 * step / 2.0);

        for oy in 0..rows {
            for ox in 0..cols {
                let points = corners(x, y, step);
                let samples: [f64; 4] = [noise_fn.get(points[0]),
                                         noise_fn.get(points[1]),
                                         noise_fn.get(points[2]),
                                         noise_fn.get(points[3])];

                let case_rows = if unicode {
                    CASES_UNICODE[samples_to_idx(&samples, threshold)]
                } else {
                    CASES[samples_to_idx(&samples, threshold)]
                };

                rb.print(ox * 2, oy * 2,
                         rustbox::RB_NORMAL, Color::White, Color::Black,
                         case_rows[0]);
                rb.print(ox * 2, (oy * 2)+1,
                         rustbox::RB_NORMAL, Color::White, Color::Black,
                         case_rows[1]);

                x += step;
            }
            y += step;
            x = startx - cols as f64 * step / 2.0;
        }

        rb.present();

        if let Ok(Event::KeyEvent(key)) = rb.poll_event(false) {
            match key {
                Key::Char('w') | Key::Up    => starty -= step,
                Key::Char('s') | Key::Down  => starty += step,
                Key::Char('a') | Key::Left  => startx -= step,
                Key::Char('d') | Key::Right => startx += step,

                Key::Char('+') if step > 0.01 => step -= 0.002,
                Key::Char('-') => step += 0.002,
                Key::Char('[') => threshold -= 0.005,
                Key::Char(']') => threshold += 0.005,

                Key::Char('u') => unicode = !unicode,

                Key::Esc | Key::Char('q') => running = false,

                _ => {}
            }
        }
    }
}
