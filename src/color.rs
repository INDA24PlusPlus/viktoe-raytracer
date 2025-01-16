use std::{fs::File, io::Write, ops::{Add, Mul}};

#[derive(Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64
}

impl Color {
    pub fn write(&self, file: &mut File) {
        file.write_fmt(format_args!(
            "{} {} {}\n",
            (self.red * 255.0).round(), (self.green * 255.0).round(), (self.blue * 255.0).round()
        ))
        .unwrap()
    }

    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color {red, green, blue}
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}
