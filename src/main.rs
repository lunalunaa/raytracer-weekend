use color::Color;
use ppm::PPM;

mod color;
mod ppm;
mod vector;

fn main() {
    let w = 256;
    let h = 256;

    let mut ppm = PPM::new(h, w);
    for i in 0..h {
        for j in 0..w {
            let color = Color::new(i as f64 / (w - 1) as f64, j as f64 / (h - 1) as f64, 0.);

            ppm.data[i][j] = color.to_rgb();
        }
    }

    ppm.export_ppm("image.ppm").unwrap();
}
