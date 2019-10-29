use super::{Pos, Tile};
///
/// Hold a 2D vector of `T`s
/// 
/// Primarily used for dungeon generation and cellular automatons, also it looks better than `Vec<Vec<T>>` and is more
/// intuitive as to what it actually represents
/// 
pub type Grid<T> = Vec<Vec<T>>;

// Make `Grid` indexable by a Pos
impl std::ops::Index<Pos> for Grid<Tile> {
  type Output = Tile;
  fn index(&self, idx: Pos) -> &Self::Output {
    &self[idx.x as usize][idx.y as usize]
  }
}

impl std::ops::IndexMut<Pos> for Grid<Tile> {
  fn index_mut(&mut self, idx: Pos) -> &mut Tile {
    &mut self[idx.x as usize][idx.y as usize]
  }
}

// Create a trait to allow the grid to have a width() and height() method
pub trait Measurable {
  fn width(&self) -> usize;
  fn height(&self) -> usize;
}

impl<T> Measurable for Grid<T> {

  fn width(&self) -> usize {
    self.len()
  }

  fn height(&self) -> usize {
    self[0].len()
  }

}