use crate::frame::{Frame, Pixel};
use rand::{Rng, SeedableRng};

pub trait LoopState {
    fn update(self) -> Self;
    fn draw<const W: usize, const H: usize>(&self, frame: Frame<W, H>) -> Frame<W, H>;
}

pub struct ExampleLoopState<const W: usize, const H: usize> {
    chem_a: Vec<f32>,
    chem_b: Vec<f32>,
}

impl<const W: usize, const H: usize> ExampleLoopState<W, H> {
    pub fn new() -> Self {
        let chem_a: Vec<f32> = vec![0.0; W * H];
        let chem_b: Vec<f32> = vec![0.0; W * H];
        Self { chem_a, chem_b }
    }

    pub fn new_seeded(seed: u64) -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let chem_a: Vec<f32> = (0..W * H).map(|_| rng.gen_range(0.0..1.0)).collect();
        let chem_b: Vec<f32> = (0..W * H).map(|_| rng.gen_range(0.0..1.0)).collect();
        Self { chem_a, chem_b }
    }
}

// Add the generic parameters to the `ExampleLoopState` struct in the trait implementation
impl<const WIDTH: usize, const HEIGHT: usize> LoopState for ExampleLoopState<WIDTH, HEIGHT> {
    fn update(self) -> Self {
        self
    }

    fn draw<const W: usize, const H: usize>(&self, mut frame: Frame<W, H>) -> Frame<W, H> {
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
