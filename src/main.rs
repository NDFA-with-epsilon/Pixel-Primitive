//grid of 64 x 48 pixels
//5 pixel wide circle in the center i.e r = 5
//eq of circle : (x-32)^2 + (y-24)^2 = r^2
// '.' for pixels inside and on the boundary of circle. rest pixels to be shown as 'x'

mod ppm;

use ppm::PPM;
use primitive_pixel::{Pixels, PointPixel, GRID_C, GRID_R};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// need error handling on this; .unwrap() functionality or something
fn render_circle(pixels: &mut Pixels, radius: usize) {
    // C(x, y) = (GRID_R/2, GRID_C/2)
    let mut coords: Vec<(usize, usize)> = Vec::new(); //coordinates to be crossed

    let mut diff_x: usize = 0;
    let mut diff_y: usize = 0;

    for (i, row) in pixels.matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if GRID_R / 2 > i {
                diff_x = GRID_R / 2 - i;
            } else {
                diff_x = i - GRID_R / 2;
            }

            if GRID_C / 2 > j {
                diff_y = GRID_C / 2 - j;
            } else {
                diff_y = j - GRID_C / 2;
            }

            if diff_x * diff_x + diff_y * diff_y <= radius * radius {
                coords.push((i, j));
            }
        }
    }

    pixels.switch_pixel(&coords, PointPixel::Cross);
    pixels.render();
}

fn main() {
    let mut grid: Pixels = Pixels::new();

    //render_circle(&mut grid, 5);
    let mut ppm_buf = PPM::new(3).unwrap();

    let mut h_img = PPM::open_ppm_file("image.ppm");

    //write the headers and other stuff to the "h_img" .ppm file
    ppm_buf.write_to_ppm(&mut h_img);
}
