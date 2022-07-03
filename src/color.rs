#[derive(Clone, Eq, PartialEq)]
pub struct Color {
  red: u8,
  green: u8,
  blue: u8
}

impl Color {
  pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
    Self { red, green, blue }
  }
}

#[macro_export]
macro_rules! rgb {
  ($red:expr, $green:expr, $blue:expr) => {
    color_str::color::Color::rgb($red, $green, $blue)
  }
}