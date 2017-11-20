use core::object::{Entity, RGB};

pub trait Renderable {

  #[inline]
  fn get_bg(&self) -> RGB;

  #[inline]
  fn get_fg(&self) -> RGB;

  #[inline]
  fn get_glyph(&self) -> char;

  #[inline]
  fn set_bg(&mut self, bg: RGB);

  #[inline]
  fn set_fg(&mut self, fg: RGB);

  #[inline]
  fn set_glyph(&mut self, glyph: char);

}

pub trait RenderableEntity: Entity + Renderable {}