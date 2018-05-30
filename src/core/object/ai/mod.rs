//!
//! Metapackage to extend an interface to ai
//! 

pub mod ai;
pub use self::ai::AI;

// 
// Ai behaviors are inherited from specific objects that have the AI trait
//

pub mod simple;
pub use self::simple::SimpleAI;

pub mod blink;
pub use self::blink::BlinkAI;

pub mod talker;
pub use self::talker::TalkerAI;

pub mod tracker;
pub use self::tracker::TrackerAI;