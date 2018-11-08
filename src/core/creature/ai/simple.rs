extern crate rand;
use self::rand::Rng;

use core::world::dungeon::map::{self, tile, Tile};

use core::creature::ai::{AI, RANDOM_TRIES};
use core::creature::{Actions, Creature, Actor, Stats};

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
  fn take_turn(&mut self, map: &map::Grid<Tile>, _player: &Creature, me: &mut Actor, _stats: &mut Stats) -> Actions {

    let mut rng = rand::thread_rng();
    let mut dice : usize;
    let mut state = Actions::Move;
    
    let mut x : usize;
    let mut y : usize;
    let mut count : usize = 0;

    // Try to move around until we find a good spot to land
    loop {

      count += 1;
      x = me.pos.x as usize;
      y = me.pos.y as usize;
      dice = rng.gen_range(1, 5);

      if x == 0 || y == 0 {
        x += 1;
        y += 1;
      }

      // Match dice for movement
      match dice {
        1 => x += 1,
        2 => x -= 1,
        3 => y += 1,
        4 => y -= 1,
        _ => unreachable!("SimpleAI - Unreachable dice state reached in movement")
      }

      // Since the only thing this thing can do is move, there is no need to match the dice again to determine state
      
      // If we find a good tile, great, otherwise keep trying until we get tired of it
      if tile::walkable(&map[x][y]) {
        break;
      } else if count > RANDOM_TRIES {
        x = me.pos.x as usize;
        y = me.pos.y as usize;
        state = Actions::Wait;
        break; 
      }

    }
    
    me.pos.x = x as isize;
    me.pos.y = y as isize;

    return state;

  }

  ///
  /// Allow Box<AI> cloning
  ///
  fn box_clone(&self) -> Box<AI> {
    Box::new((*self).clone())
  }

}