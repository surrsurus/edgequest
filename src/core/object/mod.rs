//!
//! Metapackage to hold all object submodules together
//! 

pub mod entity;
pub use self::entity::Entity;

pub mod fighter;
pub use self::fighter::Fighter;

pub mod pos;
pub use self::pos::Pos;

pub mod renderable;
pub use self::renderable::Renderable;
pub use self::renderable::RenderableEntity;

pub mod rgb;
pub use self::rgb::RGB;

mod object_tests;