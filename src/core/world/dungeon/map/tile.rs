use core::object::{Entity, RGB};

// Used to darken tiles that are out of sight
pub const DARKEN_FAC : RGB = RGB(10, 10, 10);
// Used to lighten tiles that are in the FoV
pub const YELLOW_FAC : RGB = RGB(27, 24, 22);

///
/// Traps have types
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TrapType {
  MemoryLoss
}

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
  Unseen,
  Trap(TrapType),
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
/// Scents
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ScentType {
  Player,
  Insectoid,
  Mammalian
}
// This is mmmmmm not good
pub const SCENT_TYPES : usize = 3;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Scent {
  pub val: u8,
  pub scent_type: ScentType
}

impl Scent {

  #[inline]
  pub fn new(value: u8, scent_type: ScentType) -> Scent {
    Scent {
      val: value,
      scent_type: scent_type
    }
  }

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
  pub scents: Vec<Scent>,
  pub sound: u8, // Not in use (yet)
  pub tiletype: TileType,
  pub seen: bool
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
      scents: vec![Scent::new(0, ScentType::Player), Scent::new(0, ScentType::Insectoid), Scent::new(0, ScentType::Mammalian)],
      sound: 0,
      tiletype: tiletype,
      seen: false
    }
  }

  ///
  /// Modify a tile's fg and bg color
  ///
  pub fn amplify_col(&mut self, factor: RGB) -> Tile {
    let mut t = self.clone();
    t.fg = self.fg + factor;
    t.bg = self.bg + factor;
    return t;
  }

  ///
  /// Modify a tile's fg and bg color
  ///
  pub fn reduce_col(&mut self, factor: RGB) -> Tile {
    let mut t = self.clone();
    t.fg = self.fg - factor;
    t.bg = self.bg - factor;
    return t;
  }

  ///
  /// Darken a tile's fg and bg color
  ///
  pub fn darken(&mut self) -> Tile {
    self.reduce_col(DARKEN_FAC)
  }

  ///
  /// Make a tile's fg and bg color more yellowish
  ///
  pub fn yellowish(&mut self) -> Tile {
    self.amplify_col(YELLOW_FAC)
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