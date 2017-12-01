use core::ai::AI;

use core::dungeon::map::Grid;
use core::dungeon::map::Tile;

use core::object::Fighter;

pub struct Creature {
  pub fighter: Fighter,
  pub ai: Box<AI>
}

impl Creature {

  pub fn new<T: AI + 'static>(name: String, glyph: char, pos: (isize, isize), fg: (u8, u8, u8), bg: (u8, u8, u8), ai: T) -> Creature {
    Creature {
      fighter: Fighter::new(name, glyph, pos, fg, bg),
      ai: Box::new(ai)
    }
  }

  pub fn take_turn(&mut self, map: &Grid<Tile>, player: &Fighter) {
    self.ai.take_turn(map, player, &mut self.fighter);
  }

}