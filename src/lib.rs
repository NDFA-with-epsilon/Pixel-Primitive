/*
    GRID_R: Horizontal length of the grid in which pixels are to be rendered
    GRID_C: Vertical length of the grid in which pixels are to be rendered
*/



pub const GRID_R: usize = 64;
pub const GRID_C: usize = 48;

/*
    @PointPixel:
    Defines the two values for a pixel that can be rendered
*/

pub enum PointPixel {
    Dot,
    Cross,
}

/*
    @Pixels:
    Defines a 2D array that stores u8 values: '.' (dot) and 'x' (cross)
*/

pub struct Pixels {
    pub matrix: [[char; GRID_C]; GRID_R],
}

//need error handling on these methods
impl Pixels {
    pub fn new() -> Self {
        Self {
            matrix: [['.'; GRID_C]; GRID_R],
        }
    }

    pub fn render(&self) {
        for (_, row) in self.matrix.iter().enumerate() {
            for (_, col) in row.iter().enumerate() {
                print!("{} ", col);
            }

            println!();
        }
    }

    //need error handling on these functions
    pub fn cross_pixel(&mut self, coords: &Vec<(usize, usize)>) {
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

    pub fn dot_pixel(&mut self, coords: &Vec<(usize, usize)>) {
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

    pub fn switch_pixel(&mut self, coords: &Vec<(usize, usize)>, pixel: PointPixel) {
        match pixel {
            PointPixel::Dot => self.dot_pixel(coords),
            PointPixel::Cross => self.cross_pixel(coords),
        }
    }
}
