use crate::color::Color;

pub enum Block {
  Color(Color)
}

impl IntoIterator for Block {
  type Item = Color;
  type IntoIter = BlockIterator;

  fn into_iter(self) -> Self::IntoIter {
    match self {
      Block::Color(color) => Self::IntoIter::new_color(color)
    }
  }
}

pub struct BlockIterator {
  state: BlockIteratorState
}

impl BlockIterator {
  fn new_color(color: Color) -> Self {
    let state = BlockIteratorState::Color(color);
    Self { state }
  }
}

impl Iterator for BlockIterator {
  type Item = Color;

  fn next(&mut self) -> Option<Self::Item> {
    let state = self.state.clone();
    match state {
      BlockIteratorState::None => None,
      BlockIteratorState::Color(color) => {
        self.state = BlockIteratorState::None;
        Some(color)
      },
    }
  }
}

#[derive(Clone)]
pub enum BlockIteratorState {
  None,
  Color(Color)
}