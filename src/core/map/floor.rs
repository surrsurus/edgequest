use core::map::{Grid, Scent, ScentMap, Tile};

pub use core::object::Entity;

///
/// Hold the current floor, including tiles and entities.
/// 
pub struct Floor {
  pub width: usize,
  pub height: usize,
  pub scent_map: ScentMap,
  pub tile_vec: Grid<Tile>,
}

impl Floor {

  ///
  /// Return a new `Floor`
  /// 
  #[inline]
  pub fn new(width: usize, height: usize, tile_vec: Grid<Tile>) -> Floor {

    // Make a grid
    let mut grid : ScentMap = ScentMap(Vec::new());

    // Fill it with Vecs
    for _x in 0..width {

      // Fill new vecs with walls
      let mut vec = Vec::<Scent>::new();

      for _y in 0..height {
        vec.push(Scent::new());
      }

      grid.0.push(vec);

    }

    Floor { 
      width: width, 
      height: height, 
      scent_map: grid,
      tile_vec: tile_vec,
    }
  }

}