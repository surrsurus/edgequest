use tcod::colors::Color;

pub use object::entity::Entity;
pub use object::pos::Pos;

pub struct Tile {
  pub entity: Entity,
  pub blocks: bool,
}

impl Tile {

  pub fn new(pos: Pos, glyph: char, fg: Color, bg: Color, blocks: bool) -> Tile {
    return Tile { 
      entity: Entity::new(pos, glyph, fg, bg), 
      blocks: blocks
    };
  }

}