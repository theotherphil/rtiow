
use std::convert::AsRef;
use std::fs::{File, remove_file};
use std::io::{Result, Write};
use std::path::Path;

/// RGB image.
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image { width, height, data: vec![0; 3 * width * height] }
    }

    pub fn get(&self, x: usize, y: usize) -> [u8; 3] {
        let p = 3 * y * self.width + 3 * x;
        [self.data[p], self.data[p + 1], self.data[p + 2]]
    }

    pub fn set(&mut self, x: usize, y: usize, c: [u8; 3]) {
        let p = 3 * y * self.width + 3 * x;
        self.data[p] = c[0];
        self.data[p + 1] = c[1];
        self.data[p + 2] = c[2];
    }

    pub fn save_to_ppm<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        if path.exists() {
            remove_file(path)?;
        }
        let mut output = File::create(path)?;
        writeln!(output, "P3")?;
        writeln!(output, "{} {}", self.width, self.height)?;
        writeln!(output, "255")?;
        for y in 0..self.height {
            for x in 0..self.width {
                let p = self.get(x, y);
                writeln!(output, "{} {} {}", p[0], p[1], p[2])?;
            }
        }
        Ok(())
    }
}
