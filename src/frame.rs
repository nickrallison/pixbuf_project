#![feature(generic_const_exprs)]

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]

pub struct Pixel {
    pub argb: u32,
}

impl crate::Pixel {
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        let mut pix = crate::Pixel { argb: 0 };
        pix.set_color(a, r, g, b);
        pix
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.argb = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
    }

    pub fn get_color(&self) -> (u8, u8, u8, u8) {
        let r = (self.argb & 0x00FF0000) as u8;
        let g = ((self.argb & 0x0000FF00) >> 8) as u8;
        let b = ((self.argb & 0x000000FF) >> 16) as u8;
        let a = ((self.argb & 0xFF000000) >> 24) as u8;
        (r, g, b, a)
    }
}

pub struct Frame<const W: usize, const H: usize> {
    pixels: Vec<crate::Pixel>,
}

impl<const W: usize, const H: usize> crate::Frame<W, H> {
    pub fn new() -> Self {
        crate::Frame {
            pixels: vec![crate::Pixel::new(0, 0, 0, 0); W * H],
        }
    }

    #[inline(always)]
    pub fn get_pixel(&self, x: usize, y: usize) -> &crate::Pixel {
        &self.pixels[y * W + x]
    }

    #[inline(always)]
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut crate::Pixel {
        &mut self.pixels[y * W + x]
    }

    #[inline(always)]
    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: crate::Pixel) {
        self.pixels[y * W + x] = pixel;
    }
    #[inline(always)]
    pub fn set_pixel_owned(mut self, x: usize, y: usize, pixel: crate::Pixel) -> Self {
        self.pixels[y * W + x] = pixel;
        self
    }

    #[inline(always)]
    pub fn get_pixels(&self) -> Vec<u32> {
        self.pixels.iter().map(|pix| pix.argb).collect()
    }
}
