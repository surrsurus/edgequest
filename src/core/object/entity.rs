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
  fn get_name(&self) -> String;

  #[inline]
  fn set_bg(&mut self, bg: RGB);

  #[inline]
  fn set_fg(&mut self, fg: RGB);

  #[inline]
  fn set_glyph(&mut self, glyph: char);

  #[inline]
  fn set_name(&mut self, name: String);

}