#![feature(generic_const_exprs)]

mod eventloop;
mod frame;

use crate::eventloop::{LoopState, ReactionDiffusion};
use crate::frame::{Frame, Pixel};
use minifb::{Key, Window, WindowOptions};

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    const FILL_RATE: f32 = 0.055;
    const KILL_RATE: f32 = 0.062;

    const A_DIFFUSE: f32 = 1.0;
    const B_DIFFUSE: f32 = 0.45;

    const TIME_STEP: f32 = 1.0;

    const SEED: u64 = 1;

    let mut frame = Frame::<WIDTH, HEIGHT>::new();
    let mut loopstate = ReactionDiffusion::new(
        WIDTH, HEIGHT, FILL_RATE, KILL_RATE, A_DIFFUSE, B_DIFFUSE, TIME_STEP, SEED,
    );
    frame = loopstate.draw::<WIDTH, HEIGHT>(frame);

    let mut window = Window::new(
        "Pixel Buffer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~480 fps update rate
    window.set_target_fps(480);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Convert the frame buffer to a format minifb understands
        loopstate = loopstate.update();
        frame = loopstate.draw::<WIDTH, HEIGHT>(frame);
        let buffer: Vec<u32> = frame.get_pixels();

        // Update the window with the buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
