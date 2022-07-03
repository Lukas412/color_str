use crate::block::Block;
use crate::color::Color;

pub struct ColorIterator<I: Iterator<Item=Block>> {
  sequence: I,
}