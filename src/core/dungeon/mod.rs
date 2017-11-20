//!
//! Generate a super simple dungeon
//!

extern crate rand;
use self::rand::{thread_rng, Rng};

pub mod automata;
use self::automata::{Automaton, DrunkardsWalk};

pub mod builder;
use self::builder::{Buildable, Fussy, Simple};

use core::map::{Grid, Tile};

use core::object::{Pos, RGB};

mod dungeon_tests;

/// 
/// `Dungeon` struct to stitch together all builders and cellular automatons
/// 
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Dungeon {
  pub grid: Grid<Tile>
}

impl Dungeon {

  ///
  /// Get the player's starting location as a `Pos`
  /// 
  /// NOTE: Should be deprecated and removed once stairs show up
  /// 
  pub fn get_starting_location(&self) -> Pos {
    let mut rng = thread_rng();
    let mut x : usize = rng.gen_range(1, self.grid.0.len() - 2);
    let mut y : usize = rng.gen_range(1, self.grid.0[0].len() - 2);
    while self.grid.0[x][y].blocks == true {
      x = rng.gen_range(1, self.grid.0.len() - 2);
      y = rng.gen_range(1, self.grid.0[0].len() - 2);
    };

    return Pos::from_usize(x, y);

  }

  /// 
  /// Return a new `Dungeon`
  /// 
  pub fn new(w: usize, h: usize) -> Dungeon {

    // Make a grid
    let mut grid : Grid<Tile> = Grid(vec![]);

    // Fill it with Vecs
    for _x in 0..w {

      // Fill new vecs with walls
      let mut vec = Vec::<Tile>::new();

      for _y in 0..h {
        vec.push(Tile::new(
          "Wall".to_string(),
          ' ',
          RGB(255, 255, 255), 
          RGB(33, 33, 33), 
          true
        ));
      }

      grid.0.push(vec);

    }

    return Dungeon { grid: grid };

  }

  ///
  /// Make the dungeon
  /// 
  pub fn build(&mut self) -> Grid<Tile> {

    // Apply simple builder first
    let mut grid = Simple::new(self.grid.clone()).build();

    let wall = Tile::new(
      "Wall".to_string(),
      ' ',
      RGB(255, 255, 255), 
      RGB(33, 33, 33), 
      true
    );

    let floor = Tile::new(
      "Floor".to_string(),
      ' ',
      RGB(255, 255, 255), 
      RGB(0, 0, 0), 
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
    // let mut f = Fussy::new(grid, 1.2);
    // grid = f.build();

    self.grid = grid;

    return self.grid.clone();

  }

}