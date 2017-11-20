//!
//! Metapackage to expose an interface to get cellular automatons
//! 

pub mod automaton;
pub use self::automaton::Automaton;

pub mod drunkards_walk;
pub use self::drunkards_walk::DrunkardsWalk;