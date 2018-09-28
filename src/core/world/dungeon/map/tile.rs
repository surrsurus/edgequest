use std::slice::Iter;

use core::renderer::{Entity, RGB};

use std::fmt;

// Used to darken tiles that are out of sight
pub const DARKEN_FAC : RGB = RGB(10, 10, 10);
// Used to lighten tiles that are in the FoV
pub const YELLOW_FAC : RGB = RGB(27, 24, 22);

///
/// Tiles have types
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
  Wall(Wall),
  Floor(Floor),
  Stair(Stair),
  TallGrass,
  Vine,
  Water,
  Unseen,
  // There are many different types of traps, so include them all
  Trap(Trap),
  Debug
}

///
/// Floors have types
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Floor {
  Normal,
  Crystal
}

///
/// Walls have types
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Wall {
  Normal,
  Crystal,
  Hard
}

///
/// Traps have types
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Trap {
  // NOTE: The only trap I can think about implementing right now, which just causes the player to lose all their
  // map information. Kind of just a tech demo, but it's not implemented right now
  MemoryLoss
}

///
/// Stairs have types
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Stair {
  DownStair(DownStair),
  UpStair(UpStair)
}

///
/// Up/Down stairs have types
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DownStair {
  Normal
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UpStair {
  Normal
}

///
/// Properties
/// 

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Props {
  Visibility(Visibility),
  Traversability(Traversability)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Visibility {
  Opaque,
  Transparent
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Traversability {
  Walkable,
  Blocking
}

///
/// Tile type helper functions
/// 
/// The idea is that important or reusable matching patterns can be placed into these functions for a broad range
/// of other resources to utilize without needing to update all of those patterns individually.
/// 
/// They are located in this file, as when tile types are added, the developer also ideally updates these lists at the same
/// time, meaning new tile types can be introduced swiftly
/// 

// Does the tile block vision?
pub fn opaque(t: &Tile) -> bool {
  match t.tiletype {
    Type::Wall(_) | Type::TallGrass => true,
    _ => false
  }
}

// Is it okay to spawn stuff on this tile / replace it?
pub fn spawnable(t: &Tile) -> bool {
  match t.tiletype {
    Type::Floor(_) | Type::Water | Type::TallGrass | Type::Vine => true,
    _ => false
  }
}

// Is the tile able to be walked on?
pub fn walkable(t: &Tile) -> bool {
  match t.tiletype {
    Type::Floor(_) | Type::Water | Type::Stair(_) | Type::Trap(_) | Type::TallGrass | Type::Vine => true,
    _ => false
  }
}

///
/// Archetypal floor patterns
/// 

pub fn generic_floor() -> Tile {
  Tile::new(
    "Generic Floor",
    ' ',
    RGB(0, 0, 0),
    RGB(0, 0, 0),
    Type::Floor(Floor::Normal)
  )
}

pub fn generic_wall() -> Tile {
  Tile::new(
    "Generic Wall",
    ' ',
    RGB(0, 0, 0),
    RGB(0, 0, 0),
    Type::Wall(Wall::Normal)
  )
}

///
/// Tiles have biomes
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Biome {
  Dungeon,
  Crypt,
  Cave,
  Sunken,
  Crystal
}

// Implement ability to turn the enum into a string
impl fmt::Display for Biome {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Biome::Dungeon => write!(f, "Dungeon"),
      Biome::Crypt => write!(f, "Crypt"),
      Biome::Cave => write!(f, "Cave"),
      Biome::Sunken => write!(f, "Sunken"),
      Biome::Crystal => write!(f, "Crystal")
    }
  }
}

///
/// Scents
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Scent {
  Player = 0,
  Insectoid,
  Canine,
  Feline,
  Num
}

// Implement ability to turn the enum into a string
impl fmt::Display for Scent {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Scent::Player => write!(f, "Player"),
      Scent::Insectoid => write!(f, "Insectoid"),
      Scent::Canine => write!(f, "Canine"),
      Scent::Feline => write!(f, "Feline"),
      Scent::Num => write!(f, "Num - Something wrong must have happened"),
    }
  }
}

// Implement an iterator for Scent to get the variants out in order
impl Scent {
  pub fn iterator() -> Iter<'static, Scent> {
    static SCENT_TYPES: [Scent;  Scent::Num as usize] = [
        Scent::Player, 
        Scent::Insectoid, 
        Scent::Canine, 
        Scent::Feline
      ];
      SCENT_TYPES.into_iter()
  }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct _Scent {
  pub val: u8,
  pub scent_type: Scent
}

impl _Scent {

  #[inline]
  pub fn new(value: u8, scent_type: Scent) -> _Scent {
    _Scent {
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
  pub scents: Vec<_Scent>,
  pub sound: u8,
  pub tiletype: Type,
  pub seen: bool
}

impl Tile {

  ///
  /// Return a new `Tile`
  /// 
  #[inline]
  pub fn new(name: &'static str, glyph: char, fg: RGB, bg: RGB, tiletype: Type) -> Tile {
    Tile { 
      name: name,
      glyph: glyph,
      fg: fg,
      bg: bg,
      biome: Biome::Dungeon,
      // Create scents by iterating over ScentTypes
      scents: {
        let mut sv = vec![];
        for s in Scent::iterator() {
          sv.push(_Scent::new(0, s.clone()));
        }
        sv
      },
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