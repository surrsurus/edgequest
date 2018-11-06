//!
//! Metapackage to extend an interface to ai
//! 

// 
// Ai behaviors are inherited from specific objects that have the AI trait
//

// How many times should AI randomly try stuff
// Since there will probably be a lot of AI, and since each one might be doing stuff randomly,
// the larger this gets, the more it impacts performance in the absolute worst case
pub const RANDOM_TRIES : usize = 10;

// How far away the player has to be in order for the AI to talk.
// NOTE: Probably going to get rid of this at some point
pub const TALK_DISTANCE: f32 = 20.0;

pub mod blink;
pub use self::blink::BlinkAI;

pub mod player;
pub use self::player::PlayerAI;

pub mod simple;
pub use self::simple::SimpleAI;

pub mod smeller;
pub use self::smeller::SmellerAI;

pub mod talker;
pub use self::talker::TalkerAI;

pub mod tracker;
pub use self::tracker::TrackerAI;

use core::world::dungeon::map::{self, Tile};

use core::creature::{Actions, Creature, Actor, Stats};

// As AI becomes more complex it might be a good idea to put 'general' functions in this file to help guide and maintain
// certain 'motifs' of AI such as boundary checking, creature overlap checking, etc.

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