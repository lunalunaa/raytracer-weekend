use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Result;

use crate::common::RGB;

pub struct Ppm {
    r: u32,
    c: u32,
    data: Vec<Vec<RGB>>,
}

impl Ppm {
    pub fn export_ppm(&self, path: &Path) -> Result<()> {
        let file = File::open(path)?;
        let mut buff = BufWriter::new(file);
        buff.write(b"P3\n")?;

        for i in 0..self.r {
            for j in 0..self.c {
                let pixel = self.data[i as usize][j as usize];
                buff.write(
                    (pixel.0.to_string() + " " + &pixel.1.to_string() + " " + &pixel.2.to_string())
                        .as_bytes(),
                )?;
                if j != self.c - 1 {
                    buff.write(b" ")?;
                }
                buff.write(b"\n")?;
            }
        }
        Ok(())
    }
}
