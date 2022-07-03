use crate::color::Color;

pub enum Block {
  Color(Color)
}

impl IntoIterator for Block {
  type Item = Color;
  type IntoIter = BlockIterator;

  fn into_iter(self) -> Self::IntoIter {
    match self {
      Block::Color(color) => Self::IntoIter::Color(Some(color))
    }
  }
}

pub enum BlockIterator {
  Color(Option<Color>)
}

impl Iterator for BlockIterator {
  type Item = Color;

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      BlockIterator::Color(Some(color)) => {
        let color = color.clone();
        self.0 = None;
        Some(color)
      },
      BlockIterator::Color(None) => None
    }
  }
}