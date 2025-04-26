use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Result;

use crate::common::RGB;

pub struct PPM {
    pub h: usize,
    pub w: usize,
    pub data: Vec<Vec<RGB>>,
}

impl PPM {
    pub fn new(h: usize, w: usize) -> Self {
        let data = vec![vec![RGB::default(); w]; h];
        Self { h, w, data }
    }

    pub fn export_ppm<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut buff = BufWriter::new(file);
        write!(buff, "P3\n")?;
        write!(buff, "{} {}\n", self.w, self.h)?;
        write!(buff, "255\n")?;

        for i in 0..self.h {
            for j in 0..self.w {
                let pixel = self.data[i][j];
                write!(buff, "{} {} {}\n", pixel.0, pixel.1, pixel.2)?;
            }
        }
        Ok(())
    }
}
