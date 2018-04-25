use core::object::{Entity, RGB};

///
/// Tiles have types
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TileType {
  Wall,
  Floor,
  DownStair,
  UpStair,
  Water,
  Debug
}

///
/// Tiles have biomes
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Biome {
  Dungeon
}

///
/// Tile represents an environmental entity
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Tile {
  name: &'static str,
  pub glyph: char,
  fg: RGB,
  bg: RGB,
  pub biome: Biome,
  pub scent: u8,
  pub sound: u8, // Not in use (yet)
  pub tiletype: TileType
}

impl Tile {

  ///
  /// Return a new `Tile`
  /// 
  #[inline]
  pub fn new(name: &'static str, glyph: char, fg: (u8, u8, u8), bg: (u8, u8, u8), tiletype: TileType) -> Tile {
    Tile { 
      name: name,
      glyph: glyph,
      fg: RGB::from_tup(fg),
      bg: RGB::from_tup(bg),
      biome: Biome::Dungeon,
      scent: 0,
      sound: 0,
      tiletype: tiletype
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