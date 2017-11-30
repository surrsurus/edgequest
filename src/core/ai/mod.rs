//!
//! Metapackage to extend an interface to ai
//! 

pub mod ai;
pub use self::ai::AI;

pub mod player;
pub use self::player::Player;

pub mod simple;
pub use self::simple::SimpleAI;