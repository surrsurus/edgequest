//!
//! An entity is something that can be rendered to the screen, that is:
//!
//! * Has a glyph (Character the `Entity` shows up as on the screen)
//! * Has a name (Allows us to log specific things about the `Entity`)
//! * Has a `Pos` (Allows us to know where on the map an `Entity` is)
//! * Has `RGB` foreground and backgrounds (Allows us to make it pretty)
//!
//! The entity trait simply allows us to define some getters and setters that should
//! be uniform across all `Entity`s.
//! 
//! Entities should fundamentally exist divorced from whatever rendering library we use in order to keep everything
//! the way it is with as minimal hassle as possible if a different renderer is chosen. However they are heavily
//! intertwined with the renderer and thus a bridge between game constructs and the screen.
//!

pub use core::renderer::RGB;

/// 
/// Holds a position and a character.
/// 
pub trait Entity {

  #[inline]
  fn get_bg(&self) -> RGB;

  #[inline]
  fn get_fg(&self) -> RGB;

  #[inline]
  fn get_glyph(&self) -> char;

  #[inline]
  fn get_name(&self) -> &'static str;

  #[inline]
  fn set_bg(&mut self, bg: RGB);

  #[inline]
  fn set_fg(&mut self, fg: RGB);

  #[inline]
  fn set_glyph(&mut self, glyph: char);

  #[inline]
  fn set_name(&mut self, name: &'static str);

}