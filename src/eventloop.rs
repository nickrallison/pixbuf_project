use crate::frame::{Frame, Pixel};
use rand::{Rng, SeedableRng};

pub trait LoopState {
    fn update(self) -> Self;
    fn draw<const W: usize, const H: usize>(&self, frame: Frame<W, H>) -> Frame<W, H>;
}

struct Field<const W: usize, const H: usize> {
    cells: Vec<f32>,
}

impl<const W: usize, const H: usize> Field<W, H> {
    fn new() -> Self {
        Self {
            cells: vec![0.0; W * H],
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<f32> {
        if x < W && y < H {
            Some(self.cells[y * W + x])
        } else {
            None
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, value: f32) -> Option<()> {
        if x < W && y < H {
            self.cells[y * W + x] = value;
            Some(())
        } else {
            None
        }
    }

    fn laplacian(&self, x: usize, y: usize) -> Option<f32> {
        let mut sum = 0.0;

        if x < W && y < H {
            let mut weight: f32 = 0.0;
            if x != 0 {
                sum += self.get_cell(x - 1, y).unwrap_or(0.0) * 0.2;
                weight += -0.2;
            }
            if let Some(value) = self.get_cell(x + 1, y) {
                sum += value * 0.2;
                weight += -0.2;
            }
            if y != 0 {
                sum += self.get_cell(x, y - 1).unwrap_or(0.0) * 0.2;
                weight += -0.2;
            }
            if let Some(value) = self.get_cell(x, y + 1) {
                sum += value * 0.2;
                weight += -0.2;
            }

            if x != 0 && y != 0 {
                sum += self.get_cell(x - 1, y - 1).unwrap_or(0.0) * 0.05;
                weight += -0.05;
            }
            if x != 0 && y != H - 1 {
                sum += self.get_cell(x - 1, y + 1).unwrap_or(0.0) * 0.05;
                weight += -0.05;
            }

            if x != W - 1 && y != 0 {
                sum += self.get_cell(x + 1, y - 1).unwrap_or(0.0) * 0.05;
                weight += -0.05;
            }
            if x != W - 1 && y != H - 1 {
                sum += self.get_cell(x + 1, y + 1).unwrap_or(0.0) * 0.05;
                weight += -0.05;
            }

            sum += self.get_cell(x, y).unwrap_or(0.0) * weight;
            Some(sum)
        } else {
            None
        }
    }
}

pub struct ExampleLoopState<const W: usize, const H: usize> {
    feed_rate: f32,
    kill_rate: f32,

    diffuse_a: f32,
    diffuse_b: f32,

    time_step: f32,

    chem_a: Field<W, H>, // chem_a
    chem_b: Field<W, H>, // chem_b

    rng: rand::rngs::StdRng, // SeedableRng
}

impl<const W: usize, const H: usize> ExampleLoopState<W, H> {
    pub fn new(
        feed_rate: f32,
        kill_rate: f32,
        diffuse_a: f32,
        diffuse_b: f32,
        time_step: f32,
        seed: u64,
    ) -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let mut chem_a = Field::<W, H>::new();
        chem_a.cells.fill_with(|| 1.0);

        let mut chem_b = Field::<W, H>::new();
        chem_b.cells.fill_with(|| 0.0);

        // find a random cell in B and set it and neightbours to 1.0
        let (x, y) = (rng.gen_range(0..W), rng.gen_range(0..H));

        chem_b.set_cell(x, y, 1.0);
        chem_b.set_cell(x - 1, y, 1.0);
        chem_b.set_cell(x + 1, y, 1.0);
        chem_b.set_cell(x, y - 1, 1.0);
        chem_b.set_cell(x, y + 1, 1.0);

        Self {
            feed_rate,
            kill_rate,

            diffuse_a,
            diffuse_b,

            time_step,

            chem_a,
            chem_b,

            rng,
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> LoopState for ExampleLoopState<WIDTH, HEIGHT> {
    fn update(mut self) -> Self {
        // Temporary fields to store the updated concentrations
        let mut new_chem_a = Field::<WIDTH, HEIGHT>::new();
        let mut new_chem_b = Field::<WIDTH, HEIGHT>::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                // Get current concentrations
                let a = self.chem_a.get_cell(x, y).unwrap_or(0.0);
                let b = self.chem_b.get_cell(x, y).unwrap_or(0.0);

                // Calculate the Laplacian for diffusion
                let laplacian_a = self.chem_a.laplacian(x, y).unwrap_or(0.0);
                let laplacian_b = self.chem_b.laplacian(x, y).unwrap_or(0.0);

                // Gray-Scott reaction-diffusion equations
                let reaction = a * b * b;
                let new_a = a
                    + (self.diffuse_a * laplacian_a - reaction + self.feed_rate * (1.0 - a))
                        * self.time_step;
                let new_b = b
                    + (self.diffuse_b * laplacian_b + reaction
                        - (self.kill_rate + self.feed_rate) * b)
                        * self.time_step;

                // Update the new concentrations
                new_chem_a.set_cell(x, y, new_a);
                new_chem_b.set_cell(x, y, new_b);
            }
        }

        // Update the fields with the new concentrations
        self.chem_a = new_chem_a;
        self.chem_b = new_chem_b;

        self
    }

    fn draw<const W: usize, const H: usize>(&self, mut frame: Frame<W, H>) -> Frame<W, H> {
        for y in 0..H {
            for x in 0..W {
                // Get the concentration of chemical B for visualization
                let b = self.chem_b.get_cell(x, y).unwrap_or(0.0);

                // Map the concentration to a grayscale value
                let intensity = (b * 255.0) as u8;
                let pixel = Pixel::new(intensity, intensity, intensity, 255);
                frame.set_pixel(x, y, pixel);
            }
        }
        frame
    }
}
