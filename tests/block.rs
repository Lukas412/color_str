use color_str::block::Block;
use color_str::rgb;

#[test]
fn test_none_block() {
  let block = Block::new_none();
  let mut iterator = block.into_iter();
  assert!(iterator.next().is_none())
}

#[test]
fn test_color_block() {
  let color = rgb!(0, 0, 0);
  let block = Block::new_color(color.clone());
  let mut iterator = block.into_iter();

  let next = iterator.next();
  assert!(next.is_some());
  assert!(next.unwrap() == color);

  let next = iterator.next();
  assert!(next.is_none())
}