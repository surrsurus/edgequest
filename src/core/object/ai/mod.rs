//!
//! Metapackage to extend an interface to ai
//! 

pub mod ai;
pub use self::ai::AI;

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

pub mod simple;
pub use self::simple::SimpleAI;

pub mod smeller;
pub use self::smeller::SmellerAI;

pub mod blink;
pub use self::blink::BlinkAI;

pub mod talker;
pub use self::talker::TalkerAI;

pub mod tracker;
pub use self::tracker::TrackerAI;