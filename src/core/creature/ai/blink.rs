extern crate rand;

use self::rand::Rng;

use core::world::dungeon::map::{self, Measurable, Pos, tile, Tile};

use super::{AI, RANDOM_TRIES};
use core::creature::{Actions, Creature, Actor, Stats};

const BLINK_RANGE : isize = 8;

///
/// BlinkAI makes monster teleport around the map periodically
///
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct BlinkAI;

impl BlinkAI {

  ///
  /// Return a `Pos` to a random tile nearby
  /// 
  pub fn blink(&mut self, me: &mut Actor) -> Pos {
    let mut rng = rand::thread_rng();
    let mut pos = me.pos.clone();
    pos += Pos::new(rng.gen_range(-BLINK_RANGE, BLINK_RANGE), rng.gen_range(-BLINK_RANGE, BLINK_RANGE));
    return pos;
  }

  ///
  /// Return a new AI
  /// 
  pub fn new() -> Self {
    BlinkAI {}
  }

}

impl AI for BlinkAI {
  
  ///
  /// Walk around randomly, and occasionally blink
  ///
  fn take_turn(&mut self, map: &map::Grid<Tile>, _player: &Creature, me: &mut Actor, _stats: &mut Stats) -> Actions {

    let mut rng = rand::thread_rng();

    me.prev_pos = me.pos.clone();
    
    let mut pos = me.pos.clone();
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
        1 => pos.x += 1,
        2 => pos.x -= 1,
        3 => pos.y += 1,
        4 => pos.y -= 1,
        // Blink
        5 => {
          pos = self.blink(me);
          state = Actions::Blink;
        },
        // If the rng breaks something is very wrong
        _ => unreachable!("BlinkAI - Unreachable dice state reached in movement")
      }

      // Check map bounds of previous action since this AI can pretty much just glitch straight OOB via
      // what amounts to this game's version of the BLJ
      if pos.x < 0 {
        pos.x = 0;
      }
      if pos.y < 0 {
        pos.y = 0;
      }
      if pos.y >= (map.height() - 1) as isize {
        pos.y = (map.height() - 1) as isize;
      }
      if pos.x >= (map.width() - 1) as isize {
        pos.x = (map.width() - 1) as isize;
      }

      if tile::walkable(&map[pos.x as usize][pos.y as usize]) {
        break;
      // If we make a lot of attempts and still can't find a tile::walkable tile, just stop
      } else if count > RANDOM_TRIES {
        pos = me.pos.clone();
        state = Actions::Wait;
        break; 
      }

    }
    
    // Update creature position
    me.pos = pos;

    return state;

  }

  ///
  /// Allow Box<AI> cloning
  ///
  fn box_clone(&self) -> Box<dyn AI> {
    Box::new((*self).clone())
  }

}