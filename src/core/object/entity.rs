use core::tcod::colors;

pub use core::object::pos::Pos;
pub use core::object::rgb::RGB;

/// 
/// Holds a position and a character.
/// 
/// Used for everything, basically, since all things in order to
/// be rendered need 1) a place to be rendered on the screen
/// and 2) a character to represent them on the screen.
///
/// * `pos` - `Pos` representing where the entity is on the map
/// * `glyph` - Character to represent entity on screen
/// * `fg` - Triple representing the foreground color RGB values
/// * `bg` - Triple representing the background color RGB values
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Entity {
  pub pos: Pos,
  pub glyph: char,
  // We make these triples so that we can derive Eq for this struct
  // because tcod colors don't, and if we want 2d vecs of tiles
  // they need to have Eq
  pub fg: RGB,
  pub bg: RGB,
}

impl Entity {

  ///
  /// Get the background triple as a `tcod::Color`
  /// 
  #[inline]
  pub fn get_bg(&self) -> colors::Color {
    return colors::Color::new(self.bg.0, self.bg.1, self.bg.2);
  }

  ///
  /// Get the foreground triple as a `tcod::Color`
  /// 
  #[inline]
  pub fn get_fg(&self) -> colors::Color {
    return colors::Color::new(self.fg.0, self.fg.1, self.fg.2);
  }

  /// 
  /// Move the `Entity` by `x` in the x direction and `y` in
  /// the y direction.
  /// 
  /// This does not overwrite the positon, only add to it.
  /// `x` and `y` can be positive or negative.
  /// 
  /// * `x` - ammount to move in the x direction
  /// * `y` - ammount to move in the x direction
  /// 
  pub fn move_cart(&mut self, x: i32, y: i32) {
    self.pos = Pos::new(self.pos.x + x, self.pos.y + y);
  }

  /// 
  /// Move the `Entity` by adding a new `Pos` to it
  /// 
  /// This does not overwrite the positon, only add to it.
  /// If values in `Pos` are negative, 
  /// this will then just subtract the appropriate values.
  /// 
  /// * `pos` - `Pos` struct of ammount to
  /// move in both x and y directions 
  ///  
  pub fn move_pos(&mut self, pos: Pos) {
    self.pos = self.pos + pos;
  }

  ///
  /// Return a new `Entity`
  /// 
  pub fn new(pos: Pos, glyph: char, fg: RGB, bg: RGB) -> Entity {
    return Entity {
      pos: pos, 
      glyph: glyph, 
      fg: fg, 
      bg: bg
    };
  }

  #[inline]
  pub fn set_char(&mut self, glyph: char) {
    self.glyph = glyph;
  }

  #[inline]
  pub fn set_pos(&mut self, pos: Pos) {
    self.pos = pos
  }

}
