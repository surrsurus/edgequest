//!
//! Metapackage to hold all object submodules together
//! 

pub mod entity;
pub use self::entity::Entity;

pub mod floor;
pub use self::floor::Floor;

pub mod pos;
pub use self::pos::Pos;

pub mod rgb;
pub use self::rgb::RGB;

pub mod tile;
pub use self::tile::Tile;

mod object_tests;