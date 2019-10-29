extern crate rand;

use self::rand::Rng;

use core::world::dungeon::map::{self, Measurable, tile, Tile};

use super::{AI, RANDOM_TRIES};
use core::creature::{Actions, Creature, Actor, Stats};

///
/// SmellerAI is an AI that follows insect smells
///
/// NOTE: This is a proof of concept AI
///
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SmellerAI;

impl SmellerAI {
  pub fn new() -> Self {
    SmellerAI {}
  }
}

impl AI for SmellerAI {
  
  ///
  /// Walk around randomly until it picks up a scent
  ///
  fn take_turn(&mut self, map: &map::Grid<Tile>, _player: &Creature, me: &mut Actor, _stats: &mut Stats) -> Actions {

    let mut state = Actions::Wait;
    
    let mut x = me.pos.x as usize;
    let mut y = me.pos.y as usize;

    let mut tx : isize = -1;
    let mut ty : isize = -1;
    let mut scent_val : isize = 0;

    // Represents how far the ai can smell
    let mut scent_range = 2;

    me.prev_pos = me.pos.clone();
  
    // Avoid OOB Errors
    if x < 2 || x > map.width() - 2 || y < 2 || y > map.height() - 2 {
      // NOTE: scent_range one 1 has weird behaviors...
      scent_range = 1;
    }

    for sx in x-scent_range..x+scent_range {
      for sy in y-scent_range..y+scent_range {
        // Scents[1] refers to the insectoid smell via the c-like enum
        // not the best solution
        if map[sx][sy].scents[1].val as isize > scent_val && (sx, sy) != (x, y) { 
          tx = sx as isize; 
          ty = sy as isize; 
          scent_val = map[sx][sy].scents[1].val as isize;
        }
      }
    }

    // If a scent has been picked up behave like a tracker and move towards that tile
    if scent_val > 0 && scent_val < 80{

      // Move x
      if x < tx as usize {
        x += 1;
        state = Actions::Move;
      } else if x > tx as usize {
        x -= 1;
        state = Actions::Move;
      }

      // Check
      if !tile::walkable(&map[x][y]) {
        x = tx as usize;
        state = Actions::Move;
      }

      // Move y
      if y < ty as usize{
        y += 1;
      } else if y > ty as usize {
        y -= 1;
      }

      // Check
      if !tile::walkable(&map[x][y]) {
        y = ty as usize;
        state = Actions::Move;
      }

      me.pos.x = x as isize;
      me.pos.y = y as isize;
      
    } else {
      
      // Otherwise behave like a simple ai and walk around randomly
      loop {

        let mut rng = rand::thread_rng();
        let dice : usize;
        state = Actions::Move;
        
        let mut count : usize = 0;

        count += 1;
        x = me.pos.x as usize;
        y = me.pos.y as usize;
        dice = rng.gen_range(1, 5);

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

    }

    return state;

  }

  ///
  /// Allow Box<AI> cloning
  ///
  fn box_clone(&self) -> Box<dyn AI> {
    Box::new((*self).clone())
  }

}