//!
//! `Stats` is a struct intended to hold a set of mutable data that 
//! describes the statistics of `Creature`s
//!
//! NOTE: This is terribly under-implemented on purpose, I don't want to make
//! any serious attempts at what a `Creature` needs to have in order to create a fun
//! combat system, and as of right now this is pretty much just a lot of fluff for a scent holder
//!

use core::world::dungeon::map::tile;

#[derive(Clone, Debug)]
pub struct Stats {
  // Sense
  pub perception: isize,
  pub olfaction: isize,
  // Body
  pub fortitude: isize,
  pub agility: isize,
  // Mind
  pub reason: isize,
  pub insight: isize,
  // Finite
  pub health_points: isize,
  pub sanity_points: isize,
  // Armor
  pub armor_value: isize,
  pub evasion_value: isize,
  // Weight
  pub weight: usize,
  // Scent
  pub scent_type: tile::Scent
}

impl Stats {

  ///
  /// Debug stat block
  /// 
  pub fn debug_new(weight: usize, scent_type: tile::Scent) -> Stats {
    Stats {
      perception: 0, olfaction: 0, 
      fortitude: 0, agility: 0, 
      reason: 0, insight: 0, 
      health_points: 0, sanity_points: 0, 
      armor_value: 0, evasion_value: 0,
      weight: weight, scent_type: scent_type
    }
  }

  ///
  /// Get a new Stat block
  /// 
  pub fn new(
    perception: isize,
    olfaction: isize,
    fortitude: isize,
    agility: isize,
    reason: isize,
    insight: isize,
    health_points: isize,
    sanity_points: isize,
    armor_value: isize,
    evasion_value: isize,
    weight: usize,
    scent_type: tile::Scent
  ) -> Stats {
    Stats {
      perception,
      olfaction,
      fortitude,
      agility,
      reason,
      insight,
      health_points,
      sanity_points,
      armor_value,
      evasion_value,
      weight,
      scent_type
    }
  }
}