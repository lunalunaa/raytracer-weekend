use color::RGB;
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
            let r = i as f64 / (w as f64 - 1.0);
            let g = j as f64 / (h as f64 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            ppm.data[i][j] = RGB::new(ir, ig, ib);
        }
    }

    ppm.export_ppm("image.ppm").unwrap();
}
