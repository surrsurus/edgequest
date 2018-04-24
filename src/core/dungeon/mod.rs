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
use self::map::{Grid, Tile};

use core::object::Entity;

mod dungeon_tests;

/// 
/// `Dungeon` struct to stitch together all builders and cellular automatons
/// 
#[derive(Default)]
pub struct Dungeon {
  pub width: usize,
  pub height: usize,
  pub grid: Grid<Tile>,
}

impl Dungeon {

  fn generate_grid<T : Clone>(w: usize, h: usize, init: T) -> Grid<T> {
    // Make grid
    let mut grid = Grid::<T>::new();

    // Fill it with Vecs
    for _x in 0..w {

      // Fill new vecs with init
      let mut vec = Vec::<T>::new();

      for _y in 0..h {
        vec.push(init.clone());
      }

      grid.push(vec);

    }

    return grid;

  }

  ///
  /// Get the player's starting location as a `Pos`
  /// 
  /// NOTE: Should be deprecated and removed once stairs show up
  /// 
  pub fn get_valid_location(grid: &Grid<Tile>) -> (usize, usize) {
    let mut rng = thread_rng();
    loop {
      let x : usize = rng.gen_range(1, grid.len() - 2);
      let y : usize = rng.gen_range(1, grid[0].len() - 2);
      if !grid[x][y].blocks { return (x, y) }
    }
  }

  /// 
  /// Return a new `Dungeon`
  /// 
  pub fn new(map_dim: (usize, usize)) -> Dungeon {
    let g = Dungeon::generate_grid(map_dim.0, map_dim.1, Tile::new(
        "Wall".to_string(),
        ' ',
        (255, 255, 255), 
        (33, 33, 33), 
        true));

    return Dungeon { 
      width: map_dim.0,
      height: map_dim.1,
      grid: g.clone(),
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

    // Closure for generating drunkards walks
    let drunk = |chaos: f32, iter: u32, grid: &mut Grid<Tile> | -> Grid<Tile> {
      let d = DrunkardsWalk::new(chaos);
      d.generate(
        grid, 
        None, 
        None, 
        Some(wall.clone()),
        floor.clone(),
        iter
      )
    };

    // Total randomness
    grid = drunk(1.0, 1000, &mut grid);

    // Semi random
    grid = drunk(0.5, 1000, &mut grid);

    // Mostly orderly
    grid = drunk(0.25, 1000, &mut grid);

    // Apply noise
    let mut f1 = Fussy::new(Dungeon::generate_grid(self.width, self.height, 0_u8), 1.2);
    let bin_grid1 = f1.build();

    for x in 0..self.width {
      for y in 0..self.height {
        if bin_grid1[x][y] == 1 {
          if grid[x][y].blocks {
            grid[x][y].set_bg((60, 50, 50));
          } else {
            grid[x][y].set_bg((35, 20, 20));
          } 
        }
      }
    }

    // Apply noise 2
    let mut f2 = Fussy::new(Dungeon::generate_grid(self.width, self.height, 0_u8), 1.2);
    let bin_grid2 = f2.build();

    for x in 0..self.width {
      for y in 0..self.height {
        if bin_grid2[x][y] == 1 {
          if grid[x][y].blocks {
            grid[x][y].set_bg((50, 50, 50));
          } else {
            grid[x][y].set_bg((20, 20, 20));
          } 
        }
      }
    }

    // Apply noise 3
    let mut f3 = Fussy::new(Dungeon::generate_grid(self.width, self.height, 0_u8), 1.4);
    let bin_grid3 = f3.build();

    for x in 0..self.width {
      for y in 0..self.height {
        if bin_grid3[x][y] == 1 {
          if grid[x][y].blocks {

          } else {
            grid[x][y].set_bg((0, 150, 150));
          } 
        }
      }
    }

    self.grid = grid;

  }

  pub fn is_valid(&self, x: usize, y: usize) -> bool {
    if x > 0 && x + 1 < self.width && y > 0 && y + 1 < self.height {
      if !self.grid[x][y].blocks {
        return true;
      }
    }
    return false;
  }

}