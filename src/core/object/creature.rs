use core::ai::AI;

use core::dungeon::map::Grid;
use core::dungeon::map::Tile;

use core::object::Fighter;

pub struct Creature<T: AI> {
  fighter: Fighter,
  ai: T
}

impl<T: AI> Creature<T> {

  pub fn new(name: String, glyph: char, pos: (isize, isize), fg: (u8, u8, u8), bg: (u8, u8, u8), ai: T) -> Creature<T> {
    Creature {
      fighter: Fighter::new(name, glyph, pos, fg, bg),
      ai: ai
    }
  }

  pub fn take_turn(&mut self, map: &Grid<Tile>, player: &Fighter) {
    self.ai.take_turn(map, player, &mut self.fighter);
  }

}