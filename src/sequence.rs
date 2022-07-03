use std::rc::Rc;
use std::vec::IntoIter;

use crate::block::{Block, BlockIterator};
use crate::color::Color;

pub struct ColorSequence {
  blocks: Vec<Block>,
}

impl ColorSequence {
  fn into_iter(self) -> ColorIterator<IntoIter<Block>> {
    let blocks_iterator = self.blocks.into_iter();
    ColorIterator::new(blocks_iterator)
  }
}

pub struct ColorIterator<I: Iterator<Item=Block>> {
  blocks_iterator: I,
  current_block_iterator: Option<BlockIterator>,
}

impl<I: Iterator<Item=Block>> ColorIterator<I> {
  pub fn new(blocks_iterator: I) -> Self {
    let current_block_iterator = None;
    Self { blocks_iterator, current_block_iterator }
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
    let next_block = self.blocks_iterator.next();
    next_block.map(Block::into_iter)
  }
}

impl<I: Iterator<Item=Block>> Iterator for ColorIterator<I> {
  type Item = Color;

  fn next(&mut self) -> Option<Self::Item> {
    let next = self.current_block_iterator.map_or(None, |mut iterator| iterator.next());
    next.or_else(|| self.next_with_next_block_iterator())
  }
}