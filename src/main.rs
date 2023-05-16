//grid of 64 x 48 pixels
//5 pixel wide circle in the center i.e r = 5
//eq of circle : (x-32)^2 + (y-24)^2 = r^2
// '.' for pixels inside and on the boundary of circle. rest pixels to be shown as 'x'

const GRID_R: usize = 64;
const GRID_C: usize = 48;

enum PointPixel {
    Dot,
    Cross,
}

struct Pixels {
    matrix: [[char; GRID_C]; GRID_R],
}

impl Pixels {
    fn new() -> Self {
        Self {
            matrix: [['.'; GRID_C]; GRID_R],
        }
    }

    fn cross_pixel(&mut self, coords: &Vec<(usize, usize)>) {
        if self.matrix.len() == 0 {
            println!("pixel grid in uninitialied");
            return;
        }

        if coords.len() == 0 {
            println!("Crossing the centre only...");
            self.matrix[GRID_R / 2 - 1][GRID_C / 2 - 1] = 'x';
            return;
        }

        self.matrix[GRID_R / 2 - 1][GRID_C / 2 - 1] = 'x';
        for it in coords {
            self.matrix[it.0 - 1][it.1 - 1] = 'x';
        }
    }

    fn dot_pixel(&mut self, coords: &Vec<(usize, usize)>) {
        if self.matrix.len() == 0 {
            println!("pixel grid in uninitialied");
            return;
        }

        if coords.len() == 0 {
            return;
        }

        for it in coords {
            self.matrix[it.0 - 1][it.1 - 1] = '.';
        }
    }

    fn switch_pixel(&mut self, coords: &Vec<(usize, usize)>, pixel: PointPixel) {
        match pixel {
            PointPixel::Dot => self.dot_pixel(coords),
            PointPixel::Cross => self.cross_pixel(coords),
        }
    }
}

// fn cross_pixel(pixels: &mut Pixels, col: usize, row: usize)
// {
//     if pixels.matrix.len() == 0
//     {
//         println!("pixel grid is uninitialied");
//         return;
//     }

//     if row > GRID_R || col > GRID_C
//     {
//         println!("Invalid coordinates");
//         return;
//     }

//     pixels.matrix[row-1][col-1] = 'x';
// }

// error handling on this; .unwrap() functionality or something
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
}

fn main() {
    let mut grid: Pixels = Pixels::new();

    // cross_pixel(&mut grid, 5, 3);
    
    render_circle(&mut grid, 5);
    
    // for (_, r) in grid.matrix.iter().enumerate() {
    //     for (_, c) in r.iter().enumerate() {
    //         print!("{} ", c);
    //     }
    //     println!();
    // }
}
