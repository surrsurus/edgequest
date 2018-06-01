//!
//! `Stats` is a struct intended to hold a set of mutable data that 
//! describes the statistics of `Creature`s
//!
//! NOTE: This is terribly underimplemented on purpose, I don't want to make
//! any serious attempts at what a `Creature` needs to have in order to create a fun
//! combat system
//!

use core::world::dungeon::map::ScentType;

#[derive(Debug, Clone)]
pub struct Stats {
  pub attack: isize,
  pub defense: isize,
  pub speed: isize,
  pub scent_type: ScentType
}

impl Stats {
  pub fn new(scent_type: ScentType) -> Stats {
    Stats {
      attack: 0,
      defense: 0,
      speed: 0,
      scent_type: scent_type
    }
  }
}