extern crate rand;
use self::rand::{thread_rng, Rng};

use core::world::dungeon::map::Grid;
use core::world::dungeon::map::{Tile, TileType};

use core::object::ai::AI;
use core::object::{Actions, Creature, Fighter};

///
/// SimpleAI is literally just an AI that walks around randomly
///
/// NOTE: There is really no intention to keep this AI around... Maybe as a confused AI?
/// Definitely will be replaced/refactored.
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SimpleAI;

impl SimpleAI {
  pub fn new() -> SimpleAI {
    SimpleAI {}
  }
}

impl AI for SimpleAI {
  
  ///
  /// Walk around randomly
  ///
  fn take_turn(&mut self, map: &Grid<Tile>, _player: &Creature, me: &mut Fighter) -> Actions {

    let mut rng = thread_rng();
    let mut dice : i32;
    let mut state = Actions::Move;
    
    let mut x : usize;
    let mut y : usize;
    let mut count : usize = 0;
    loop {
      count += 1;
      x = me.pos.x as usize;
      y = me.pos.y as usize;
      dice = rng.gen_range(1, 5);
      match dice {
        1 => x += 1,
        2 => x -= 1,
        3 => y += 1,
        4 => y -= 1,
        _ => unreachable!("dice machine broke")
      }

      match map[x][y].tiletype {
        TileType::Floor => break,
        _ => {
          if count > 100 {
            x = me.pos.x as usize;
            y = me.pos.y as usize;
            state = Actions::Wait;
            break; 
          }
        }
      }
    }
    
    me.pos.x = x as isize;
    me.pos.y = y as isize;

    return state;

  }

}