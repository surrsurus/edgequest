use core::object::{Entity, RGB};

///
/// Tile represents an environmental entity
/// 
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Tile {
  name: &'static str,
  pub glyph: char,
  pub blocks: bool,
  fg: RGB,
  bg: RGB,
  pub biome: &'static str,
  pub scent: u8,
  pub sound: u8
}

impl Tile {

  ///
  /// Return a new `Tile`
  /// 
  #[inline]
  pub fn new(name: &'static str, glyph: char, fg: (u8, u8, u8), bg: (u8, u8, u8), blocks: bool) -> Tile {
    Tile { 
      name: name,
      glyph: glyph,
      fg: RGB::from_tup(fg),
      bg: RGB::from_tup(bg),
      blocks: blocks,
      biome: "dungeon",
      scent: 0,
      sound: 0
    }
  }

}

impl Entity for Tile {

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
  fn get_name(&self) -> &'static str {
    self.name.clone()
  }

  #[inline]
  fn set_bg(&mut self, bg: (u8, u8, u8)) {
    self.bg = RGB::from_tup(bg);
  }

  #[inline]
  fn set_fg(&mut self, fg: (u8, u8, u8)) {
    self.fg = RGB::from_tup(fg);
  }

  #[inline]
  fn set_glyph(&mut self, glyph: char) {
    self.glyph = glyph
  }

  #[inline]
  fn set_name(&mut self, name: &'static str) {
    self.name = name;
  }

}