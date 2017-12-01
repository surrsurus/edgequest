use core::ai::AI;

use core::dungeon::map::Grid;
use core::dungeon::map::Tile;

use core::object::Fighter;

pub struct SimpleAI;

impl SimpleAI {
  pub fn new() -> SimpleAI {
    SimpleAI {}
  }
}

impl AI for SimpleAI {
  
  fn take_turn(&mut self, map: &Grid<Tile>, player: &Fighter, me: &mut Fighter) {
    
  }

}