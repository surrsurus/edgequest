use core::world::dungeon::map::Grid;
use core::world::dungeon::map::Tile;

use core::object::Fighter;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MovementTypes {
  Dumb,
  Track,
  Blink
}

///
/// Represents basic actions AI can take in the game
///
pub trait AI {

  ///
  /// Make the AI take it's turn based on map, player, and itself
  ///
  /// NOTE: I feel like this is going to have to change, maybe it'll take a vec of
  /// creatures instead? Definitely going to change in the future. Might even need a tcod map?
  ///
  fn take_turn(&mut self, map: &Grid<Tile>, player: &Fighter, me: &mut Fighter);

}
