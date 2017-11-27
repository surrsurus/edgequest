//!
//! Metapackage to hold all object submodules together
//! 

pub mod entity;
pub use self::entity::Entity;

pub mod fighter;
pub use self::fighter::Fighter;

pub mod pos;
pub use self::pos::Pos;

mod object_tests;