//!
//! Metapackage to hold all object submodules together
//! 
pub mod ai;

pub mod creature;
pub use self::creature::Creature;

pub mod entity;
pub use self::entity::Entity;

pub mod fighter;
pub use self::fighter::Fighter;

pub mod pos;
pub use self::pos::Pos;

pub mod rgb;
pub use self::rgb::RGB;

mod object_tests;