use crate::{ray::Interval, vector::Vec3};

#[derive(Clone, Copy, Default)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }

    pub fn as_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

pub type Color = Vec3;

impl Color {
    pub fn linear_to_gamma(linear_component: f32) -> f32 {
        if linear_component > 0. {
            linear_component.sqrt()
        } else {
            0.
        }
    }

    pub fn as_rgb(&self) -> Rgb {
        let intensity = Interval::new(0.000, 0.999);
        let r = (256.0 * intensity.clamp(Self::linear_to_gamma(self.x))) as u8;
        let g = (256.0 * intensity.clamp(Self::linear_to_gamma(self.y))) as u8;
        let b = (256.0 * intensity.clamp(Self::linear_to_gamma(self.z))) as u8;

        Rgb::new(r, g, b)
    }
}
