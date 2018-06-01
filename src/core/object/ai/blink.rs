extern crate rand;
use self::rand::{thread_rng, Rng};

use core::world::dungeon::map::Grid;
use core::world::dungeon::map::{Tile, TileType};

use core::object::{Actions, Creature, Actor};
use core::object::ai::AI;

///
/// BlinkAI makes monster teleport around the map periodically
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlinkAI;

impl BlinkAI {

  pub fn blink(&mut self, me: &mut Actor) -> (isize, isize) {
    let mut rng = thread_rng();
    let mut x = me.pos.x;
    let mut y = me.pos.y;
    x += rng.gen_range(-8, 8);
    y += rng.gen_range(-8, 8);
    return (x, y);
  }

  pub fn new() -> BlinkAI {
    BlinkAI {}
  }

}

impl AI for BlinkAI {
  
  ///
  /// Walk around randomly
  ///
  fn take_turn(&mut self, map: &Grid<Tile>, _player: &Creature, me: &mut Actor) -> Actions {

    let mut rng = thread_rng();
    
    let mut x = me.pos.x;
    let mut y = me.pos.y;
    let mut count : usize = 0;
    let mut state = Actions::Move;

    loop {
      count += 1;

      let dice : i32;
      dice = rng.gen_range(1, 6);
      match dice {
        1 => x += 1,
        2 => x -= 1,
        3 => y += 1,
        4 => y -= 1,
        5 => {
          let bpos = self.blink(me);
          x = bpos.0;
          y = bpos.1;
          state = Actions::Blink;
        },
        _ => unreachable!("dice machine broke")
      }

      // Check bounds
      if x < 0 {
        x = 0;
      }
      if y < 0 {
        y = 0;
      }
      if y >= (map[0].len() - 1) as isize {
        y = (map[0].len() - 1) as isize;
      }
      if x >= (map.len() - 1) as isize {
        x = (map.len() - 1) as isize;
      }

      match map[x as usize][y as usize].tiletype {
        TileType::Floor | TileType::Water | TileType::UpStair | TileType::DownStair => break,
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

    return state;

  }

}