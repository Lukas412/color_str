use std::rc::Rc;
use std::vec::IntoIter;

use crate::block::{Block, BlockIterator};
use crate::color::Color;
use crate::iterator::ColorIterator;

pub struct ColorSequence {
  sequence: Vec<Block>,
}

impl IntoIterator for ColorSequence {
  type Item = Block;
  type IntoIter = ColorIterator<IntoIter<Self::Item>>;

  fn into_iter(self) -> Self::IntoIter {
    let iterator = self.sequence.into_iter();
    ColorIterator::new(iterator)
  }
}

pub struct ColorIterator<I: Iterator<Item=Block>> {
  blocks: I,
  current_block_iterator: Option<BlockIterator>,
}

impl<I: Iterator<Item=Block>> ColorIterator<I> {
  pub fn new(blocks: I) -> Self {
    let current = None;
    Self { blocks, current_block_iterator: current }
  }
}

impl<I: Iterator<Item=Block>> ColorIterator<I> {
  fn next_with_next_block_iterator(&mut self) -> Option<Color> {
    self.next_block_iterator().and_then(|next_block_iterator| {
      self.current_block_iterator = Some(next_block_iterator);
      self.next()
    })
  }

  fn next_block_iterator(&mut self) -> Option<BlockIterator> {
    let next_block = self.blocks.next();
    next_block.map(Block::into_iter)
  }
}

impl<I: Iterator<Item=Block>> Iterator for ColorIterator<I> {
  type Item = Color;

  fn next(&mut self) -> Option<Self::Item> {
    let next_color = self.current_block_iterator.map(BlockIterator::next);
    next_color.or_else(ColorIterator::next_with_next_block_iterator)
  }
}