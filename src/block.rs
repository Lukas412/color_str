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

pub struct BlockIterator {
  state: BlockIteratorState
}

impl Iterator for BlockIterator {
  type Item = Color;

  fn next(&mut self) -> Option<Self::Item> {
    match &self.state {
      BlockIteratorState::None => None,
      BlockIteratorState::Color(color) => {
        self.state = BlockIteratorState::None;
        Some(color.clone())
      },
    }
  }
}

pub enum BlockIteratorState {
  None,
  Color(Color)
}