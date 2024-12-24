use crate::frame::{Frame, Pixel};

pub trait LoopState {
    fn update(self) -> Self;
    fn draw<const W:  usize, const H: usize>(&self, frame: Frame<W, H>) -> Frame<W, H>;
}

pub struct ExampleLoopState {}
impl LoopState for ExampleLoopState {
    fn update(self) -> Self {
        self
    }
    fn draw<const W:  usize, const H: usize>(&self, mut frame: Frame<W, H>) -> Frame<W, H> {
        for y in 0..H {
            for x in 0..W {
                let a = (x % 256) as u8;
                let r = (y % 256) as u8;
                let g = ((x + y) % 256) as u8;
                let b = ((x + y) % 256) as u8;
                let pixel = Pixel::new(a, r, g, b);
                frame.set_pixel(x, y, pixel);
            }
        }
        frame
    }
}