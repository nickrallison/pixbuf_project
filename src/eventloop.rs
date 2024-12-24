use crate::frame::{Frame, Pixel};

pub trait LoopState {
    fn update(self) -> Self;
    fn draw<const W: usize, const H: usize>(&self, frame: Frame<W, H>) -> Frame<W, H>;
}

pub struct ReactionDiffusion {
    width: usize,
    height: usize,
    a: Vec<f32>,
    b: Vec<f32>,
    feed_rate: f32,
    kill_rate: f32,
    diffusion_a: f32,
    diffusion_b: f32,
    delta_t: f32,
}

impl ReactionDiffusion {
    pub fn new(width: usize, height: usize, feed_rate: f32, kill_rate: f32) -> Self {
        let size = width * height;
        let mut a = vec![1.0; size];
        let mut b = vec![0.0; size];

        // Initialize a small area with B=1
        let center_x = width / 2;
        let center_y = height / 2;
        let radius = 10;
        for y in (center_y.saturating_sub(radius))..(center_y + radius) {
            for x in (center_x.saturating_sub(radius))..(center_x + radius) {
                let dx = x as isize - center_x as isize;
                let dy = y as isize - center_y as isize;
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
            diffusion_a: 1.0,
            diffusion_b: 0.5,
            delta_t: 1.0,
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

impl LoopState for ReactionDiffusion {
    fn update(self) -> Self {
        let mut new_a = self.a.clone();
        let mut new_b = self.b.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let a = self.a[index];
                let b = self.b[index];

                let reaction = a * b * b;
                let laplacian_a = self.laplacian(&self.a, x, y);
                let laplacian_b = self.laplacian(&self.b, x, y);

                new_a[index] = a
                    + (self.diffusion_a * laplacian_a - reaction + self.feed_rate * (1.0 - a))
                        * self.delta_t;
                new_b[index] = b
                    + (self.diffusion_b * laplacian_b + reaction
                        - (self.kill_rate + self.feed_rate) * b)
                        * self.delta_t;
            }
        }

        ReactionDiffusion {
            a: new_a,
            b: new_b,
            ..self
        }
    }

    fn draw<const W: usize, const H: usize>(&self, mut frame: Frame<W, H>) -> Frame<W, H> {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let a = self.a[index];
                let b = self.b[index];

                let color_value = ((1.0 - a) * 255.0) as u8;
                let pixel = Pixel::new(255, color_value, color_value, color_value);
                frame.set_pixel(x, y, pixel);
            }
        }

        frame
    }
}
