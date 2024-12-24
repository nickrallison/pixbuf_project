#![feature(generic_const_exprs)]

mod eventloop;
mod frame;

use crate::eventloop::{ExampleLoopState, LoopState};
use crate::frame::{Frame, Pixel};
use minifb::{Key, Window, WindowOptions};

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut frame = Frame::<WIDTH, HEIGHT>::new();
    let mut loopstate = ExampleLoopState::<WIDTH, HEIGHT>::new();
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

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Convert the frame buffer to a format minifb understands
        loopstate = loopstate.update();
        frame = loopstate.draw::<WIDTH, HEIGHT>(frame);
        let buffer: Vec<u32> = frame.get_pixels();

        // Update the window with the buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
