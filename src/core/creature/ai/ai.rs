use core::world::dungeon::map::{self, Tile};

use core::creature::{Actions, Creature, Actor, Stats};

///
/// Represents basic actions AI can take in the game
/// 
/// An AI is a trait because we want all AI to follow a similar pattern and thus be Boxable and able to be given
/// to `Creature`s. Thus, all AI patterns are trait objects.
///
pub trait AI {

  ///
  /// Make the AI take it's turn based on map, player, and itself
  /// 
  /// NOTE: AIs are basically just state deciders at this point but more complex AIs have to be state machines in of themselves
  /// in order to create complex behaviors. At some point they should take in a state, a vector of all creatures on the floor
  /// (for monster infighting, fight-flight) and maybe even some sort of "mood" though that would be a part of the `Creature`. I am
  /// completely considering adding randomized personalities to monsters to create even more combinations of behavior.
  ///
  fn take_turn(&mut self, map: &map::Grid<Tile>, player: &Creature, me: &mut Actor, stats: &mut Stats) -> Actions;

}