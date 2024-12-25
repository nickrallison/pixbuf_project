use crate::frame::{Frame, Pixel};
use rayon::prelude::*;

pub trait LoopState {
    fn update(self) -> Self;
    fn draw<const W: usize, const H: usize>(&self, frame: Frame<W, H>) -> Frame<W, H>;
}

pub struct ReactionDiffusion<F, K>
where
    F: Fn(f32, f32) -> f32 + Send + Sync,
    K: Fn(f32, f32) -> f32 + Send + Sync,
{
    width: usize,
    height: usize,
    a: Vec<f32>,
    b: Vec<f32>,
    feed_rate: F,
    kill_rate: K,
    a_diffusion: f32,
    b_diffusion: f32,
    delta_t: f32,
}

impl<F, K> ReactionDiffusion<F, K>
where
    F: Fn(f32, f32) -> f32 + Send + Sync,
    K: Fn(f32, f32) -> f32 + Send + Sync,
{
    pub fn new(
        width: usize,
        height: usize,
        feed_rate: F,
        kill_rate: K,
        a_diffusion: f32,
        b_diffusion: f32,
        delta_t: f32,
        start_x: usize,
        start_y: usize,
        rad: usize,
        seed: u64,
    ) -> Self {
        let size = width * height;
        let mut a = vec![1.0; size];
        let mut b = vec![0.0; size];

        // Initialize a small area with B=1
        let radius = rad;
        for y in (start_y.saturating_sub(radius))..(start_y + radius) {
            for x in (start_x.saturating_sub(radius))..(start_x + radius) {
                let dx = x as isize - start_x as isize;
                let dy = y as isize - start_y as isize;
                if dx * dx + dy * dy < (radius * radius) as isize {
                    b[y * width + x] = 1.0;
                }
            }
        }

        ReactionDiffusion {
            width,
            height,
            a,
            b,
            feed_rate,
            kill_rate,
            a_diffusion,
            b_diffusion,
            delta_t,
        }
    }

    fn laplacian(&self, grid: &Vec<f32>, x: usize, y: usize) -> f32 {
        let mut sum = 0.0;
        let index = y * self.width + x;
        let neighbors = [
            (0, 0, -1.0), // center
            (-1, 0, 0.2),
            (1, 0, 0.2), // left, right
            (0, -1, 0.2),
            (0, 1, 0.2), // top, bottom
            (-1, -1, 0.05),
            (1, -1, 0.05), // top-left, top-right
            (-1, 1, 0.05),
            (1, 1, 0.05), // bottom-left, bottom-right
        ];

        for &(dx, dy, weight) in &neighbors {
            let nx = (x as isize + dx).clamp(0, (self.width - 1) as isize) as usize;
            let ny = (y as isize + dy).clamp(0, (self.height - 1) as isize) as usize;
            sum += grid[ny * self.width + nx] * weight;
        }
        sum
    }
}

impl<F, K> LoopState for ReactionDiffusion<F, K>
where
    F: Fn(f32, f32) -> f32 + Send + Sync,
    K: Fn(f32, f32) -> f32 + Send + Sync,
{
    fn update(self) -> Self {
        let mut new_a = self.a.clone();
        let mut new_b = self.b.clone();

        let width = self.width;
        let height = self.height;
        let a_diffusion = self.a_diffusion;
        let b_diffusion = self.b_diffusion;
        let delta_t = self.delta_t;

        new_a
            .par_iter_mut()
            .zip(new_b.par_iter_mut())
            .enumerate()
            .for_each(|(index, (new_a_val, new_b_val))| {
                let x = index % width;
                let y = index / width;
                let a = self.a[index];
                let b = self.b[index];

                // Normalize x and y for feed_rate and kill_rate functions
                let nx = x as f32 / width as f32;
                let ny = y as f32 / height as f32;

                let reaction = a * b * b;
                let laplacian_a = self.laplacian(&self.a, x, y);
                let laplacian_b = self.laplacian(&self.b, x, y);

                let feed_rate = (self.feed_rate)(nx, ny);
                let kill_rate = (self.kill_rate)(nx, ny);

                *new_a_val =
                    a + (a_diffusion * laplacian_a - reaction + feed_rate * (1.0 - a)) * delta_t;
                *new_b_val = b
                    + (b_diffusion * laplacian_b + reaction - (kill_rate + feed_rate) * b)
                        * delta_t;
            });

        ReactionDiffusion {
            a: new_a,
            b: new_b,
            ..self
        }
    }

    fn draw<const W: usize, const H: usize>(&self, mut frame: Frame<W, H>) -> Frame<W, H> {
        let width = self.width;
        frame
            .pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let a = self.a[index];
                let b = self.b[index];
                let color_value = ((1.0 - b) * 255.0) as u8;
                *pixel = Pixel::new(255, color_value, color_value, color_value);
            });
        frame
    }
}
