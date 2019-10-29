//!
//! Metapackage to expose an interface to get cellular automatons
//! 

extern crate rand;
use self::rand::Rng;

use core::world::dungeon::map;
use core::world::dungeon::map::Measurable;

// Import automatons here

pub mod drunkards_walk_d4;
pub use self::drunkards_walk_d4::DrunkardsWalkD4;

///
/// `Automaton` trait to define a set of behavior for all cellular automatons
/// 
pub trait Automaton {

  /// 
  /// What type `T` of `Grid<T>` is on output and input
  /// 
  type Output : Clone;

  ///
  /// Generate the cellular automata
  /// 
  /// We take some `Grid<T>` of undefined length and width where `T` is set to `Self::Output` and
  /// operate the cellular automaton over the grid, replacing instances of `find` with
  /// `replace`. If `find` is `None`, the automaton should replace every `T` it finds with
  /// `replace`.
  /// 
  /// This is designed so that cellular automatons can operate sequentially on a grid, constantly adding to it.
  /// This is typically intended to work with `Tile`s, however it can work with any type `T`
  /// 
  /// It is implied that you should initialize each builder with it's `new()` method
  /// then call this function
  /// 
  fn apply(&self, grid: &mut map::Grid<Self::Output>, starting_pos: Option<map::Pos>, find: Option<Self::Output>, replace: Self::Output, iterations: u32);

  /// 
  /// Automatically unwrap a `Pos` in a way thats suitable for most automata
  /// 
  fn unwrap_pos(&self, grid: &map::Grid<Self::Output>, pos: Option<map::Pos>) -> map::Pos {
    match pos {
      Some(pos) => pos,
      None => map::Pos::from_usize(rand::thread_rng().gen_range(1, grid.width() - 2), rand::thread_rng().gen_range(1, grid.height() - 2))
    }
  } 

  ///
  /// Place a `Pos` within the bounds of the grid in case if it is out of bounds
  /// 
  fn place_inbounds(&self, grid: &map::Grid<Self::Output>, pos: &mut map::Pos) {
    // Check bounds, leave a gap though between the border.
    // Obviously if your grid is a 1x1 this will cause an issue.
    if pos.x < 1 { pos.x = 1; }
    if pos.y < 1 { pos.y = 1; }
    if pos.x >= (grid.width() - 2) as isize { pos.x = (grid.width() - 2) as isize; }
    if pos.y >= (grid.height() - 2) as isize { pos.y = (grid.height() - 2) as isize; }
  }

  ///
  /// Get chaos. Basically just a random number between 0 and 1
  /// 
  fn get_chaos(&self) -> f32 {
    rand::thread_rng().gen::<f32>()
  }

  ///
  /// Get a d4 for cartesian movement
  /// 
  fn get_d4(&self) -> usize {
    rand::thread_rng().gen_range(1, 5)
  }

  ///
  /// Get a d8 for all direction cartesian movement
  /// 
  fn get_d8(&self) -> usize {
    rand::thread_rng().gen_range(1, 9)
  }

  ///
  /// Get a d9 for all direction cartesian movement, plus a 9th spot to represent the currently 'stood on' tile
  /// 
  fn get_d9(&self) -> usize {
    rand::thread_rng().gen_range(1, 10)
  }

}