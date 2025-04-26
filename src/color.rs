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
