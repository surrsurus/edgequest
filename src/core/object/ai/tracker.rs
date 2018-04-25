use core::world::dungeon::map::Grid;
use core::world::dungeon::map::{Tile, TileType};

use core::object::Fighter;
use core::object::ai::AI;

///
/// AI that tracks player
///
pub struct TrackerAI;

impl TrackerAI {
  pub fn new() -> TrackerAI {
    TrackerAI {}
  }
}

impl AI for TrackerAI {
  
  ///
  /// Walk around randomly
  ///
  fn take_turn(&mut self, map: &Grid<Tile>, player: &Fighter, me: &mut Fighter) {

    let distance = me.pos ^ player.pos;

    if distance < 20.0 && distance > 2.00 {
      let mut x = me.pos.x;
      let mut y = me.pos.y;
      
      // Move x
      if x < player.pos.x {
        x += 1;
      } else if x > player.pos.x {
        x -= 1;
      }

      // Check
      match map[x as usize][y as usize].tiletype {
        TileType::Wall => x = me.pos.x,
        _ => {}
      }

      // Move y
      if y < player.pos.y {
        y += 1;
      } else if y > player.pos.y {
        y -= 1;
      }

      // Check
      match map[x as usize][y as usize].tiletype {
        TileType::Wall => y = me.pos.y,
        _ => {}
      }

      me.pos.x = x;
      me.pos.y = y;

    }

  }

}