use crate::{ray::Interval, vector::Vec3};

#[derive(Clone, Copy, Default)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(unused)]
impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}

pub type Color = Vec3;

impl Color {
    pub fn to_rgb(&self) -> RGB {
        let intensity = Interval::new(0.000, 0.999);
        let r = (256.0 * intensity.clamp(self.x)) as u8;
        let g = (256.0 * intensity.clamp(self.y)) as u8;
        let b = (256.0 * intensity.clamp(self.z)) as u8;

        RGB::new(r, g, b)
    }
}
