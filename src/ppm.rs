use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Result;

use crate::color::Rgb;

pub struct Ppm {
    pub h: usize,
    pub w: usize,
    pub data: Vec<Vec<Rgb>>,
}

impl Ppm {
    pub fn new(h: usize, w: usize) -> Self {
        let data = vec![vec![Rgb::default(); w]; h];
        Self { h, w, data }
    }

    pub fn export_ppm<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut buff = BufWriter::new(file);
        writeln!(buff, "P3")?;
        writeln!(buff, "{} {}", self.w, self.h)?;
        writeln!(buff, "255")?;

        for i in 0..self.h {
            for j in 0..self.w {
                let pixel = self.data[i][j];
                writeln!(buff, "{} {} {}", pixel.r, pixel.g, pixel.b)?;
            }
        }
        Ok(())
    }
}
