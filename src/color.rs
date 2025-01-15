use std::{fs::File, io::Write};

pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    pub fn write(&self, file: &mut File) {
        file.write_fmt(format_args!(
            "{} {} {}\n",
            self.red, self.green, self.blue
        ))
        .unwrap()
    }

    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color {red, green, blue}
    }
}
