pub use core::object::{Entity, Grid, Tile};

///
/// Hold the current floor, including tiles and entities.
/// 
pub struct Floor {
  pub width: usize,
  pub height: usize,
  pub tile_vec: Grid<Tile>,
}

impl Floor {

  ///
  /// Return a new `Floor`
  /// 
  #[inline]
  pub fn new(width: usize, height: usize, tile_vec: Grid<Tile>) -> Floor {
    Floor { 
      width: width, 
      height: height, 
      tile_vec: tile_vec,
    }
  }

}