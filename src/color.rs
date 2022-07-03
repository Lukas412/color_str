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