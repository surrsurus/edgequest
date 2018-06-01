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

use core::renderer::RGB;

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
  fn set_bg(&mut self, bg: (u8, u8, u8));

  #[inline]
  fn set_fg(&mut self, fg: (u8, u8, u8));

  #[inline]
  fn set_glyph(&mut self, glyph: char);

  #[inline]
  fn set_name(&mut self, name: &'static str);

}