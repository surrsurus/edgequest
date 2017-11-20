use core::object::{Entity, RenderableEntity, RGB, Renderable};

///
/// Tile represents an environmental entity
/// 
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Tile {
  pub name: String,
  pub glyph: char,
  pub blocks: bool,
  fg: RGB,
  bg: RGB,
  pub biome: String
}

impl Tile {

  ///
  /// Return a new `Tile`
  /// 
  #[inline]
  pub fn new(name: String, glyph: char, fg: RGB, bg: RGB, blocks: bool) -> Tile {
    Tile { 
      name: name,
      glyph: glyph,
      fg: fg,
      bg: bg,
      blocks: blocks,
      biome: "dungeon".to_string()
    }
  }

}

impl Entity for Tile {

  #[inline]
  fn get_name(&self) -> String {
    self.name.clone()
  }

  #[inline]
  fn set_name(&mut self, name: String) {
    self.name = name;
  }

}

impl Renderable for Tile {

  #[inline]
  fn get_bg(&self) -> RGB {
    self.bg
  }

  #[inline]
  fn get_fg(&self) -> RGB {
    self.fg
  }

  #[inline]
  fn get_glyph(&self) -> char {
    self.glyph
  }

  #[inline]
  fn set_bg(&mut self, bg: RGB) {
    self.bg = bg;
  }

  #[inline]
  fn set_fg(&mut self, fg: RGB) {
    self.fg = fg;
  }

  #[inline]
  fn set_glyph(&mut self, glyph: char) {
    self.glyph = glyph
  }

}

impl RenderableEntity for Tile {}