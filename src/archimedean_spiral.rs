// The Archimedean spiral is a spiral named after the 3rd-century BC Greek mathmetician Archimedes. It
// is the locus corresponding to the locations over time of a point moving away from a fixed point with
// a constant speed along a line that rotates with constant angular velocity. Equivalently, in polar
// coordinares (r, θ) it can be described by the equation: r = a + b ⋅ θ

#[macro_use(px)]
extern crate bmp;

use bmp::{ Image, Pixel };
use std::f64;

fn main() {
    let width = 600u32;
    let half_width = (width / 2) as i32;
    let mut img = Image::new(width, width);
    let draw_color = px!(255, 128, 128);

    let a = 1.0_f64;
    let b = 9.0_f64;
    let max_angle = 5.0_f64 * 2.0_f64 * f64::consts::PI;
    let mut theta = 0.0_f64;

    while theta < max_angle {
        theta += 0.002_f64;

        let r = a + b * theta;
        let x = (r * theta.cos()) as i32 + half_width;
        let y = (r * theta.sin()) as i32 + half_width;
        img.set_pixel(x as u32, y as u32, draw_color);
    }

    let _ = img.save("archimedean_spiral.bmp").unwrap_or_else(|e| panic!("Failed to save: {}", e));
}