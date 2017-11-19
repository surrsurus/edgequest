//!
//! Metapackage to hold all object submodules together
//! 

pub mod pos;
pub use self::pos::Pos;

pub mod rgb;
pub use self::rgb::RGB;

pub mod entity;
pub use self::entity::Entity;

pub mod tile;
pub use self::tile::Tile;

pub mod floor;
pub use self::floor::Floor;

pub mod grid;
pub use self::grid::Grid;

mod object_tests;