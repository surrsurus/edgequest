//!
//! Generate a super simple dungeon
//!

extern crate rand;
use self::rand::{thread_rng, Rng};

mod automata;
use self::automata::{Automaton, DrunkardsWalk};

mod builder;
use self::builder::{Buildable, Fussy, Simple};

pub mod map;
use self::map::{Grid, Scent, ScentMap, Tile};

use core::object::Entity;

mod dungeon_tests;

/// 
/// `Dungeon` struct to stitch together all builders and cellular automatons
/// 
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Dungeon {
  pub width: usize,
  pub height: usize,
  pub grid: Grid<Tile>,
  pub scent_map: ScentMap,
}

impl Dungeon {

  ///
  /// Get the player's starting location as a `Pos`
  /// 
  /// NOTE: Should be deprecated and removed once stairs show up
  /// 
  pub fn get_starting_location(&self) -> (usize, usize) {
    let mut rng = thread_rng();
    let mut x : usize = rng.gen_range(1, self.grid.0.len() - 2);
    let mut y : usize = rng.gen_range(1, self.grid.0[0].len() - 2);
    while self.grid.0[x][y].blocks == true {
      x = rng.gen_range(1, self.grid.0.len() - 2);
      y = rng.gen_range(1, self.grid.0[0].len() - 2);
    };

    return (x, y);

  }

  /// 
  /// Return a new `Dungeon`
  /// 
  pub fn new(map_dim: (usize, usize)) -> Dungeon {

    // Make grids
    let mut map_grid : Grid<Tile> = Grid(vec![]);
    let mut scent_grid : ScentMap = Grid(vec![]);

    // Fill it with Vecs
    for _x in 0..map_dim.0 {

      // Fill new vecs with walls
      let mut map_vec = Vec::<Tile>::new();
      let mut scent_vec = Vec::<Scent>::new();

      for _y in 0..map_dim.1 {
        map_vec.push(Tile::new(
          "Wall".to_string(),
          ' ',
          (255, 255, 255), 
          (33, 33, 33), 
          true
        ));
        scent_vec.push(Scent::new());
      }

      map_grid.0.push(map_vec);
      scent_grid.0.push(scent_vec);

    }

    return Dungeon { 
      width: map_dim.0,
      height: map_dim.1,
      grid: map_grid,
      scent_map: scent_grid 
    };

  }

  ///
  /// Make the dungeon
  /// 
  pub fn build(&mut self) {

    // Apply simple builder first
    let mut grid = Simple::new(self.grid.clone()).build();

    let wall = Tile::new(
      "Wall".to_string(),
      ' ',
      (255, 255, 255), 
      (33, 33, 33), 
      true
    );

    let floor = Tile::new(
      "Floor".to_string(),
      ' ',
      (255, 255, 255), 
      (0, 0, 0), 
      false
    );

    // Total randomness
    let d1 = DrunkardsWalk::new(1.0);
    grid = d1.generate(
      &mut grid, 
      None, 
      None, 
      Some(wall.clone()),
      floor.clone(),
      1000
    );

    // Semi random
    let d2 = DrunkardsWalk::new(0.5);
    grid = d2.generate(
      &mut grid, 
      None, 
      None, 
      Some(wall.clone()),
      floor.clone(),
      750
    );

    // Mostly orderly
    let d3 = DrunkardsWalk::new(0.25);
    grid = d3.generate(
      &mut grid, 
      None, 
      None, 
      Some(wall.clone()),
      floor.clone(),
      750
    );

    // Apply noise
    let mut bin_grid : Grid<u8> = Grid(vec![]);
    for _x in 0..self.width {
      let mut bin_vec = Vec::<u8>::new();
      for _y in 0..self.height {
        bin_vec.push(0);
      }
      bin_grid.0.push(bin_vec);
    }

    let mut f = Fussy::new(bin_grid, 1.2);
    bin_grid = f.build();

    for x in 0..self.width {
      for y in 0..self.height {
        if bin_grid.0[x][y] == 1 {
          if grid.0[x][y].blocks {
            grid.0[x][y].set_bg((100, 100, 60));
          } else {
            grid.0[x][y].set_bg((50, 50, 40));
          } 
        }
      }
    }

    self.grid = grid;

  }

}