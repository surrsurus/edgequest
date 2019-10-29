extern crate rand;
use self::rand::Rng;

use core::renderer::RGB;

use super::Filter;

use super::map::{Grid, Measurable, tile, Tile};

// Configuration

// Should be divisible by 2
pub const RADIUS : usize = 4;

///
/// Viscera generator
/// 
/// Place a slightly randomized viscera on a certain point
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Viscera;

impl Viscera {

  ///
  /// Add viscera on a tile
  ///
  pub fn viscerize(x: usize, y: usize, grid: &mut Grid<Tile>) {

    let mut rng = rand::thread_rng();

    // Coinflip to decide if a tile gets blood
    if rng.gen_range(0, 2) == 0 {
      // Match walls or floors only
      match grid[x - (RADIUS/2)][y - (RADIUS/2)].tiletype {
        tile::Type::Wall(_) => {
          grid[x - (RADIUS/2)][y - (RADIUS/2)].fg += RGB(22 + rng.gen_range(0, 40), rng.gen_range(0, 10), 0);
          grid[x - (RADIUS/2)][y - (RADIUS/2)].bg += RGB(12 + rng.gen_range(0, 50), rng.gen_range(0, 10), 0);
        },
        tile::Type::Floor(_) => {
          grid[x - (RADIUS/2)][y - (RADIUS/2)].fg += RGB(rng.gen_range(0, 40), rng.gen_range(0, 10), 0);
          grid[x - (RADIUS/2)][y - (RADIUS/2)].bg += RGB(rng.gen_range(0, 50), rng.gen_range(0, 10), 0);
        },
        _ => {}
      }
    }

  }

  ///
  /// Add some viscera on the map randomly
  /// 
  fn add_blood_random(&self, grid: &mut Grid<Tile>) {

    let mut rng = rand::thread_rng();

    for _ in 0..rng.gen_range(5, 15) {

      // Read details of grid
      let total_w = grid.width();
      let total_h = grid.height();

      // Get a random x y
      let x = rng.gen_range(RADIUS, total_w - RADIUS - 1);
      let y = rng.gen_range(RADIUS, total_h - RADIUS - 1);

      // So we want to cover a 3x3 area and randomly decide if there is blood there and what color it will be
      for i in 0..RADIUS {
        for j in 0..RADIUS {
          Viscera::viscerize(x + i, y + j, grid);
        }
      }

    }

  }

  /// 
  /// Return a new `Viscera`
  /// 
  pub fn new() -> Self {

    return Viscera {}

  }

}

impl Filter for Viscera {

  type Output = Tile;

  fn apply(&mut self, grid: &mut Grid<Self::Output>) {

    debugln!("viscera", "spreading gore randomly...");

    // Generate the dungeon
    self.add_blood_random(grid);

  }

}