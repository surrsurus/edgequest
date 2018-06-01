extern crate rand;
use self::rand::{thread_rng, Rng};

use core::GlobalLog;

use core::world::dungeon::map::Grid;
use core::world::dungeon::map::Tile;

use core::object::ai::AI;
use core::object::{Actions, Creature, Fighter, Entity};

///
/// AI that tracks player
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TalkerAI;

impl TalkerAI {
  #[inline]
  pub fn new() -> TalkerAI {
    TalkerAI {}
  }
}

impl AI for TalkerAI {
  
  ///
  /// Track player if near
  ///
  fn take_turn(&mut self, _map: &Grid<Tile>, player: &Creature, me: &mut Fighter) -> Actions {

    let mut state = Actions::Wait;

    let distance = me.pos ^ player.fighter.pos;

    if distance < 20.0 {

      // Mutable reference to the mutex
      let mut log = GlobalLog.lock().unwrap();

      let mut rng = thread_rng();
      let dice : i32 = rng.gen_range(1, 15);;
      match dice {
        1...10 => (),
        11 => log.push(("'This is where we live'", me.get_fg())),
        12 => log.push(("'This is where we get work done'", me.get_fg())),
        13 => log.push(("'Don't touch the arrow keys'", me.get_fg())),
        14 => log.push(("'Talk to the TAs'", me.get_fg())),
        _ => unreachable!("dice machine broke")
      }

      drop(log);
      
      state = Actions::Talk;

    }

    return state;

  }

}