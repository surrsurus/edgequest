use game::tcod::colors::Color;

pub use game::object::entity::Entity;
pub use game::object::pos::Pos;

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
  pub fn new(pos: Pos, glyph: char, fg: (u8, u8, u8), bg: (u8, u8, u8), blocks: bool) -> Tile {
    return Tile { 
      entity: Entity::new(pos, glyph, fg, bg), 
      blocks: blocks
    };
  }

}