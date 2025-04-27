use crate::vector::Vec3;

#[derive(Clone, Copy)]
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

impl Default for RGB {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

pub type Color = Vec3;

impl Color {
    pub fn to_rgb(&self) -> RGB {
        let r = (255.999 * self.x) as u8;
        let g = (255.999 * self.y) as u8;
        let b = (255.999 * self.z) as u8;

        RGB::new(r, g, b)
    }
}
