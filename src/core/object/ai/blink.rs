use core::world::dungeon::map::Grid;
use core::world::dungeon::map::{Tile, TileType};

use core::object::Fighter;
use core::object::ai::AI;
use core::object::ai::MovementTypes;

extern crate rand;
use self::rand::{thread_rng, Rng};

///
/// BlinkAI makes monster teleport around the map periodically
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlinkAI {
  properties: Vec<MovementTypes>
}

impl BlinkAI {
  pub fn new() -> BlinkAI {
    BlinkAI {
      properties: vec![MovementTypes::Blink]
    }
  }
}

impl AI for BlinkAI {
  
  ///
  /// Walk around randomly
  ///
  fn take_turn(&mut self, map: &Grid<Tile>, _player: &Fighter, me: &mut Fighter) {

    let mut rng = thread_rng();
    let mut dice : i32;
    
    let mut x : isize;
    let mut y : isize;
    let mut count : usize = 0;
    loop {
      count += 1;
      x = me.pos.x;
      y = me.pos.y;
      dice = rng.gen_range(1, 6);
      match dice {
        1 => x += 1,
        2 => x -= 1,
        3 => y += 1,
        4 => y -= 1,
        5 => {
          x += rng.gen_range(-8, 8);
          y += rng.gen_range(-8, 8);
          if x < 0 {
            x = 0;
          }
          if y < 0 {
            y = 0;
          }
          if y > map[0].len() as isize {
            y = map[0].len() as isize;
          }
          if x > map.len() as isize {
            x = map.len() as isize;
          }
        },
        _ => unreachable!("dice machine broke")
      }

      match map[x as usize][y as usize].tiletype {
        TileType::Floor => break,
        _ => {
          if count > 100 {
            x = me.pos.x;
            y = me.pos.y;
            break; 
          }
        }
      }
    }
    
    me.pos.x = x as isize;
    me.pos.y = y as isize;

  }

}