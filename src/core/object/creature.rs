use core::world::dungeon::map::Grid;
use core::world::dungeon::map::{Tile, ScentType};

use core::object::ai::AI;
use core::object::{Actions, Actor, Stats};

///
/// Creature holds a `Actor` and an `AI`, basically a package that we can create monsters from
///
pub struct Creature {
  pub actor: Actor,
  pub stats: Stats,
  pub state: Actions,
  pub ai: Box<AI>
}

impl Creature {

  #[inline]
  pub fn new<T: AI + 'static>(name: &'static str, glyph: char, pos: (isize, isize), fg: (u8, u8, u8), bg: (u8, u8, u8), scent_type: ScentType, ai: T) -> Creature {
    Creature {
      actor: Actor::new(name, glyph, pos, fg, bg),
      stats: Stats::new(scent_type),
      state: Actions::Unknown,
      ai: Box::new(ai)
    }
  }

  pub fn take_turn(&mut self, map: &Grid<Tile>, player: &Creature) {
    self.state = self.ai.take_turn(map, player, &mut self.actor);
  }

}