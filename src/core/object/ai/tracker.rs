use core::world::dungeon::map::Grid;
use core::world::dungeon::map::{Tile, TileType};

use core::object::ai::AI;
use core::object::{Actions, Creature, Actor};

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
  /// Track player if near
  ///
  fn take_turn(&mut self, map: &Grid<Tile>, player: &Creature, me: &mut Actor) -> Actions {

    let mut state = Actions::Wait;

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
      match map[x as usize][y as usize].tiletype {
        TileType::Wall => x = me.pos.x,
        _ => {}
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
      match map[x as usize][y as usize].tiletype {
        TileType::Wall => y = me.pos.y,
        _ => {}
      }

    }
    
    me.pos.x = x as isize;
    me.pos.y = y as isize;

    return state;

  }

}