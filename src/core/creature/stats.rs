//!
//! `Stats` is a struct intended to hold a set of mutable data that 
//! describes the statistics of `Creature`s
//!
//! NOTE: This is terribly underimplemented on purpose, I don't want to make
//! any serious attempts at what a `Creature` needs to have in order to create a fun
//! combat system, and as of right now this is pretty much just a lot of fluff for a scent holder
//!

use core::world::dungeon::map::tile;

#[derive(Clone, Debug)]
pub struct Stats {
  pub attack: usize,
  pub defense: usize,
  pub speed: usize,
  pub weight: usize,
  pub scent_type: tile::Scent
}

impl Stats {
  pub fn new(
    attack: usize,
    defense: usize,
    speed: usize,
    weight: usize,
    scent_type: tile::Scent
  ) -> Stats {
    Stats {
      attack: attack,
      defense: defense,
      speed: speed,
      weight: weight,
      scent_type: scent_type
    }
  }
}