use core::ai::AI;

use core::dungeon::map::Grid;
use core::dungeon::map::Tile;

use core::object::Fighter;

pub struct Player;

impl Player {
  pub fn new() -> Player {
    Player {}
  }
}

impl AI for Player {

  type Me = Player;
  
  fn take_turn<T: AI>(&mut self, _map: &Grid<Tile>, _player: &Fighter<T>, me: &mut Fighter<Player>) {}

}