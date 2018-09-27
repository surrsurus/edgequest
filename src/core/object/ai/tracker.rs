use core::world::dungeon::map::Grid;
use core::world::dungeon::map::{Tile, walkable};

use core::object::ai::AI;
use core::object::{Actions, Creature, Actor, Stats};

///
/// AI that tracks player
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TrackerAI;

impl TrackerAI {
  #[inline]
  pub fn new() -> TrackerAI {
    TrackerAI {}
  }
}

impl AI for TrackerAI {
  
  ///
  /// Track player and follow if near
  ///
  fn take_turn(&mut self, map: &Grid<Tile>, player: &Creature, me: &mut Actor, _stats: &mut Stats) -> Actions {

    let mut state = Actions::Wait;

    // ^ is overridden to be the distance formula, this isn't xor
    let distance = me.pos ^ player.actor.pos;
    let mut x = me.pos.x;
    let mut y = me.pos.y;

    if distance < 20.0 && distance > 2.00 {

      // Move x
      if x < player.actor.pos.x {
        x += 1;
        state = Actions::Move;
      } else if x > player.actor.pos.x {
        x -= 1;
        state = Actions::Move;
      }

      // Check
      if !walkable(&map[x as usize][y as usize]) {
        x = me.pos.x;
      }

      // Move y
      if y < player.actor.pos.y {
        y += 1;
        state = Actions::Move;
      } else if y > player.actor.pos.y {
        y -= 1;
        state = Actions::Move;
      }

      // Check
      if !walkable(&map[x as usize][y as usize]) {
        y = me.pos.y
      }

    }
    
    me.pos.x = x as isize;
    me.pos.y = y as isize;

    return state;

  }

}