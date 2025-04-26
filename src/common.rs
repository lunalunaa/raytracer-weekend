#[derive(Clone, Copy)]
pub struct RGB(pub u8, pub u8, pub u8);

#[allow(unused)]
impl RGB {
    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }

    pub fn to_slice(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }
}

impl Default for RGB {
    fn default() -> Self {
        Self(0, 0, 0)
    }
}
