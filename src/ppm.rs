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
    pub magic_num: String,
    pub img_dim: [u8; 4], //[width, space, height, newline]
    pub max_val: [u8; 4], //[max_val, newline]
    pub rgb: Vec<Vec<u8>>,
    //vec<&[u8]>
}

impl PPM {
    pub fn new(encoding: u32) -> Result<Self, &'static str> {
        match encoding {
            3 => Ok(Self {
                // magic_num: ['P' as u8, encoding],
                magic_num: "P3\n".to_string(),
                img_dim: [0; 4],
                max_val: [0; 4],
                rgb: Vec::new(),
            }),

            6 => Ok(Self {
                // magic_num: ['P' as u8, encoding],
                magic_num: "P6\n".to_string(),
                img_dim: [0; 4],
                max_val: [0; 4],
                rgb: Vec::new(),
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

    pub fn write_to_buf(&mut self) {
        //assuming P3
        //set width & height of image
        //TEMP: set width = 3 and height = 2
        self.img_dim = [51, 32, 50, 10];

        //set maximum value for each color
        self.max_val = [50, 53, 53, 10];

        //now fill color values for each pixel
        // for i in 1..sz {
        //     match i {
        //         1 => self.rgb.push("255 0 0\n".to_string()),
        //         2 => self.rgb.push("0 255 0\n".to_string()),
        //         3 => self.rgb.push("0 0 255\n".to_string()),
        //         4 => self.rgb.push("255 255 0\n".to_string()),
        //         5 => self.rgb.push("255 255 255\n".to_string())
        //         6 => self.rgb.push("0 0 0\n".to_string()),
        //     }
        // }

        self.rgb.push(vec![50, 53, 53, 32, 48, 32, 48, 10]);
        self.rgb.push(vec![48, 32, 50, 53, 53, 32, 48, 10]);
        self.rgb.push(vec![48, 32, 48, 32, 50, 53, 53, 10]);
        self.rgb.push(vec![50, 53, 53, 32, 50, 53, 53, 32, 48, 10]);
        self.rgb
            .push(vec![50, 53, 53, 32, 50, 53, 53, 32, 50, 53, 53, 10]);
        self.rgb.push(vec![48, 32, 48, 32, 48]);
    }

    pub fn write_to_ppm(&mut self, h_img: &mut File) {
        //newline need not necessarily "look" like newline i.e. need not actually start from a new line when you open the image.ppm file. just insert newline character where a newline is supposed to start from
        h_img.write_all(&self.magic_num.as_bytes()).unwrap();
        println!("bytes written: header {:?}", &self.magic_num.as_bytes());
        // self.img_dim = [3, 32, 2, 10];
        h_img.write_all(&self.img_dim).unwrap();
        // println!("bytes written: dim {:?}", &self.img_dim);

        // self.max_val = [255, 10];
        h_img.write(&self.max_val).unwrap();
        // println!("bytes written: max_val {:?}", &self.max_val);

        for it in self.rgb.iter() {
            h_img.write_all(it).unwrap();
        }
    }
}
