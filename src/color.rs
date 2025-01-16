use std::{
    fs::File,
    io::Write,
    ops::{Add, Mul},
};

#[derive(Debug)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

fn linear_to_gamma(color: f64) -> f64 {
    if color > 0.0 {
        color.sqrt()
    } else {
        0.0
    }
}

impl Color {
    pub fn write(&self, file: &mut File) {
        file.write_fmt(format_args!(
            "{} {} {}\n",
            (linear_to_gamma(self.red.clamp(0.000, 0.999)) * 255.0).round(),
            (linear_to_gamma(self.green.clamp(0.000, 0.999)) * 255.0).round(),
            (linear_to_gamma(self.blue.clamp(0.000, 0.999)) * 255.0).round()
        ))
        .unwrap()
    }

    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}
