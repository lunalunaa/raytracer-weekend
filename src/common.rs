#[derive(Clone, Copy)]
pub struct RGB(pub u8, pub u8, pub u8);

impl RGB {
    pub fn to_slice(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }
}
