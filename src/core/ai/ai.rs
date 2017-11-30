use core::dungeon::map::Grid;
use core::dungeon::map::Tile;

use core::object::Fighter;

pub trait AI {

  type Me : AI;

  fn take_turn<T: AI>(&mut self, map: &Grid<Tile>, player: &Fighter<T>, me: &mut Fighter<Self::Me>);

}