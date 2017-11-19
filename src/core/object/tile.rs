pub use core::object::{Entity, Pos, RGB};

///
/// Tile represents an environmental entity
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Tile {
  pub entity: Entity,
  pub blocks: bool,
}

impl Tile {

  ///
  /// Return a new `Tile`
  /// 
  #[inline]
  pub fn new(pos: Pos, glyph: char, fg: RGB, bg: RGB, blocks: bool) -> Tile {
    Tile { 
      entity: Entity::new(pos, glyph, fg, bg), 
      blocks: blocks
    }
  }

  #[inline]
  pub fn set_pos(&mut self, pos: Pos) {
    self.entity.pos = pos;
  }

  #[inline]
  pub fn set_glyph(&mut self, glyph: char) {
    self.entity.glyph = glyph;
  }
  
  #[inline]
  pub fn set_rgbs(&mut self, fg: RGB, bg: RGB) {
    self.entity.fg = fg;
    self.entity.bg = bg;
  }

}