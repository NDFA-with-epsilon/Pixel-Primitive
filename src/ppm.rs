/*
    Definitions of structures and methods for the PPM file
*/

/*
    Current Idea:
    Open a file with .ppm extension, write into it a buffer with all headers and necessary stuff required by the PPM file format and hope it works
*/

// use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct PPM {
    /* type of file (PBM, PGM, PPM) and its enconding (ASCII or binary). For PPM: P3 and P6 */
    // pub magic_num: [u8; 2],
    pub magic_num: &'static str,
}

impl PPM {
    pub fn new(encoding: u32) -> Result<Self, &'static str> {
        match encoding {
            3 => Ok(Self {
                // magic_num: ['P' as u8, encoding],
                magic_num: "P3"
            }),

            6 => Ok(Self {
                // magic_num: ['P' as u8, encoding],
                magic_num: "P6"
            }),

            _ => Err("invalid params"),
        }
    }

    pub fn open_ppm_file(img_name: &str) -> File {
        let path = Path::new(img_name);
        let display = path.display();

        let file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why), //look for better error handling than panic!()-ing

            Ok(file) => file,
        };

        file
    }

    pub fn write_to_buf(&mut self) {}

    pub fn write_to_ppm(&mut self, h_img: &mut File) {
        h_img.write_all(self.magic_num.as_bytes()).unwrap();
    }
}
