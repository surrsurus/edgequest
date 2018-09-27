//!
//! `Stats` is a struct intended to hold a set of mutable data that 
//! describes the statistics of `Creature`s
//!
//! NOTE: This is terribly underimplemented on purpose, I don't want to make
//! any serious attempts at what a `Creature` needs to have in order to create a fun
//! combat system, and as of right now this is pretty much just a lot of fluff for a scent holder
//!

use core::world::dungeon::map::ScentType;

#[derive(Clone, Debug)]
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