
use core::creature::Creature;
use core::world::dungeon::map::{self, Tile};

///
/// Time
/// 
pub trait Time {
  
  fn take_turn(&mut self, map: &map::Grid<Tile>, player: &Creature);

}