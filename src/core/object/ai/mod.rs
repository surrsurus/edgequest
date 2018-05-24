//!
//! Metapackage to extend an interface to ai
//! 

pub mod ai;
pub use self::ai::AI;
pub use self::ai::MovementTypes;

pub mod simple;
pub use self::simple::SimpleAI;

pub mod blink;
pub use self::blink::BlinkAI;

pub mod tracker;
pub use self::tracker::TrackerAI;