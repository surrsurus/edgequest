//!
//! Metapackage to hold all object submodules together
//! 
pub mod ai;

pub mod actions;
pub use self::actions::Actions;

pub mod creature;
pub use self::creature::Creature;

pub mod entity;
pub use self::entity::Entity;

pub mod actor;
pub use self::actor::Actor;

pub mod pos;
pub use self::pos::Pos;

mod object_tests;