extern crate rand;
use self::rand::{thread_rng, Rng};

use core::world::dungeon::map::Grid;
use core::world::dungeon::map::{Tile, walkable};

use core::object::ai::{AI, RANDOM_TRIES};
use core::object::{Actions, Creature, Actor, Stats};

///
/// BlinkAI makes monster teleport around the map periodically
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlinkAI;

impl BlinkAI {

  ///
  /// Get a random tile nearby
  /// 
  pub fn blink(&mut self, me: &mut Actor) -> (isize, isize) {
    let mut rng = thread_rng();
    let mut x = me.pos.x;
    let mut y = me.pos.y;
    x += rng.gen_range(-8, 8);
    y += rng.gen_range(-8, 8);
    return (x, y);
  }

  ///
  /// Return a new AI
  /// 
  pub fn new() -> BlinkAI {
    BlinkAI {}
  }

}

impl AI for BlinkAI {
  
  ///
  /// Walk around randomly, and occasionally blink
  ///
  fn take_turn(&mut self, map: &Grid<Tile>, _player: &Creature, me: &mut Actor, _stats: &mut Stats) -> Actions {

    let mut rng = thread_rng();
    
    let mut x = me.pos.x;
    let mut y = me.pos.y;
    let mut count : usize = 0;
    // Start out in a movement state since the only other state this AI can be in is Blink or Wait
    // which are updated accordingly
    let mut state = Actions::Move;

    // Basically just check for a valid tile a bunch of times
    loop {
      count += 1;

      // Decide on walking or blinking
      let dice : usize;
      dice = rng.gen_range(1, 6);

      // Match dice for movement
      match dice {
        1 => x += 1,
        2 => x -= 1,
        3 => y += 1,
        4 => y -= 1,
        // Blink
        5 => {
          let bpos = self.blink(me);
          x = bpos.0;
          y = bpos.1;
          state = Actions::Blink;
        },
        // If the rng breaks something is very wrong
        _ => unreachable!("BlinkAI - Unreachable dice state reached in movement")
      }

      // Check map bounds of previous action since this AI can pretty much just glitch straight OOB via
      // what ammounts to this game's version of the BLJ
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

      if walkable(&map[x as usize][y as usize]) {
        break;
      // If we make a lot of attempts and still can't find a walkable tile, just stop
      } else if count > RANDOM_TRIES {
        x = me.pos.x;
        y = me.pos.y;
        state = Actions::Wait;
        break; 
      }

    }
    
    // Update creature position
    me.pos.x = x as isize;
    me.pos.y = y as isize;

    return state;

  }

}