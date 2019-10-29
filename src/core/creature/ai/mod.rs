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

use core::world::dungeon::map::{self, Measurable, Tile};

use core::creature::{Actions, Creature, Actor, Stats};

// As AI becomes more complex it might be a good idea to put 'general' functions in this file to help guide and maintain
// certain 'motifs' of AI such as boundary checking, creature overlap checking, etc.

///
/// Represents basic actions AI can take in the game
/// 
/// AIs are intended to be used as trait objects. The idea is that creatures are given some boxed
/// AI object to hold. This object is then guaranteed to have a `take_turn()` function, meaning
/// that all creatures have a uniform way of taking a turn without needing to know the exact AI
/// implementation.
/// 
/// This gives us the primary advantage of being able to swap AI objects out at will, and also allow
/// AIs to track data about themselves such as turns spent asleep, turns spent confused, etc. AIs really are just
/// the state deciders for creatures, and thus the implementation of the creature's AI and their actual codified data
/// are separate entities, which is a good thing.
/// 
/// This concept of using trait objects for their unified function is similar to abstract classes in other languages,
/// and directly creates a pseudo ECS system since AI is now simply a component of a creature.
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

  ///
  /// Determine if the AI has gone out of bounds with respect to the given map
  ///
  fn is_oob(&mut self, x: isize, y: isize, map: &map::Grid<Tile>) -> bool { 
    // Check for below map (< 0) and above map (> map.width() - 1)
    x < 0 || y < 0 || y >= (map.height() - 1) as isize || x >= (map.width() - 1) as isize
  }

  ///
  /// Allow boxed trait objects to be cloned
  /// 
  fn box_clone(&self) -> Box<dyn AI>;

}

///
/// Allow cloning of boxed trait objects via box_clone()
///
/// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714
/// 
/// The downside is that all things that impl AI need to have a very similar box clone, but that's not an issue
impl Clone for Box<dyn AI> {
  fn clone(&self) -> Box<dyn AI> {
    self.box_clone()
  }
}