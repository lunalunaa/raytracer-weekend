use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::{Ok, Result};
use image::ImageBuffer;

use crate::color::Rgb;

pub struct Canvas {
    pub h: u32,
    pub w: u32,
    pub data: Vec<Vec<Rgb>>,
}

impl Canvas {
    pub fn new(h: u32, w: u32) -> Self {
        let data = vec![vec![Rgb::default(); w as usize]; h as usize];
        Self { h, w, data }
    }

    #[allow(unused)]
    pub fn export_ppm<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut buff = BufWriter::new(file);
        writeln!(buff, "P3")?;
        writeln!(buff, "{} {}", self.w, self.h)?;
        writeln!(buff, "255")?;

        for i in 0..self.h {
            for j in 0..self.w {
                let pixel = self.data[i as usize][j as usize];
                writeln!(buff, "{} {} {}", pixel.r, pixel.g, pixel.b)?;
            }
        }
        Ok(())
    }

    pub fn export_png<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let img = ImageBuffer::from_par_fn(self.w, self.h, |x, y| {
            image::Rgb(self.data[y as usize][x as usize].as_array())
        });

        let mut buf = BufWriter::new(File::create(path)?);
        img.write_to(&mut buf, image::ImageFormat::Png)?;

        Ok(())
    }
}
