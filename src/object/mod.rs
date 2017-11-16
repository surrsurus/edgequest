//!
//! Metapackage to hold all object submodules together
//! 

pub mod pos;
pub use self::pos::Pos;

pub mod entity;
pub use self::entity::Entity;

pub mod tile;
pub use self::tile::Tile;

pub mod map;
pub use self::map::Map;

mod object_tests;