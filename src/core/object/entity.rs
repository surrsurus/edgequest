use core::object::RGB;

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