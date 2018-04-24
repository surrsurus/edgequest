use core::world::dungeon::map::Grid;
use core::world::dungeon::map::Tile;

use core::object::Fighter;

pub trait AI {

  fn take_turn(&mut self, map: &Grid<Tile>, player: &Fighter, me: &mut Fighter);

}