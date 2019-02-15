//!
//! A creature holds a state, an `AI`, some `Stats`, and an `Actor`
//!
//! The intention is that `Creature` binds all neccesary parts that an `AI` needs to function
//! such that each part is easily passed in and also easily replacable
//!

pub mod ai;

pub mod actions;
pub use self::actions::Actions;

pub mod actor;
pub use self::actor::Actor;

pub mod stats;
pub use self::stats::Stats;

mod object_tests;

use core::item::Item;
use core::renderer::RGB;
use core::world::dungeon::map::{self, Pos, tile, Tile};

///
/// Creature holds a `Actor` and an `AI`, basically a package that we can create monsters from
///
#[derive(Clone)]
pub struct Creature {
  pub actor: Actor,
  pub stats: Stats,
  pub state: Actions,

  // Q: Wait, an AI trait object is clonable?
  // A: A *Boxed* AI trait object is clonable, as pointers to objects are clonable
  pub ai: Box<ai::AI>,

  // Items
  // Hold money
  pub wallet: f32,
  // Hold other items
  pub inventory: Vec<Item>
}

impl Creature {

  ///
  /// Create a new `Creature`
  ///
  /// NOTE: This doesn't even scratch the surface of what `Creature`s should be. Here's some ideas I was considering:
  /// 
  /// Mood - How the monster is feeling
  /// Personalities - Helps determine specfic actions based off creature state and mood
  /// Inventory - What the creature is holding
  /// Illnesses - What afflictions the creature has (affects mood)
  /// 
  /// What I'm basically proposing is to first turn the game into a nature simulator first, and a game second
  ///
  #[inline]
  pub fn new<T: ai::AI + 'static>(name: &'static str, glyph: char, pos: Pos, fg: RGB, bg: RGB, scent_type: tile::Scent, ai: T) -> Self {
    Creature {
      actor: Actor::new(name, glyph, pos, fg, bg),
      stats: Stats::new(scent_type),
      state: Actions::Unknown,
      ai: Box::new(ai),
      wallet: 0.0,
      inventory: vec![]
    }
  }

  ///
  /// Passthrough to `AI`
  ///
  /// Essentially allows us to not need to include `AI` when we need to `take_turn()`
  ///
  pub fn take_turn(&mut self, map: &map::Grid<Tile>, player: &Creature) {
    self.state = self.ai.take_turn(map, player, &mut self.actor, &mut self.stats);
  }

}