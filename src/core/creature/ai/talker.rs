extern crate rand;
use self::rand::Rng;

use core::log;

use core::world::dungeon::map::{self, Tile};

use core::renderer::Renderable;

use super::{AI, TALK_DISTANCE};
use core::creature::{Actions, Creature, Actor, Stats};

///
/// AI that talks to the player
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TalkerAI;

impl TalkerAI {
  #[inline]
  pub fn new() -> Self {
    TalkerAI {}
  }
}

impl AI for TalkerAI {
  
  ///
  /// Talk to player if near
  ///
  fn take_turn(&mut self, _map: &map::Grid<Tile>, player: &Creature, me: &mut Actor, _stats: &mut Stats) -> Actions {

    let mut state = Actions::Wait;

    // ^ is overridden to be the distance formula, this isn't xor
    let distance = me.pos ^ player.actor.pos;

    if distance < TALK_DISTANCE {

      let mut rng = rand::thread_rng();
      let dice : i32 = rng.gen_range(1, 15);

      // Match dice for voiceline
      match dice {
        1...10 => (),
        11 => log!(("'This is where we live'", me.get_fg())),
        12 => log!(("'This is where we get work done'", me.get_fg())),
        13 => log!(("'Don't touch the arrow keys'", me.get_fg())),
        14 => log!(("'Talk to the TAs'", me.get_fg())),
        _ => unreachable!("TalkerAI - Unreachable dice state reached in talk")
      }

      // Match dice for action
      match dice {
        1...10 => (),
        _ => state = Actions::Talk
      }
      
    }

    return state;

  }

  ///
  /// Allow Box<AI> cloning
  ///
  fn box_clone(&self) -> Box<AI> {
    Box::new((*self).clone())
  }

}