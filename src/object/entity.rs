use tcod::colors::Color;

pub use object::pos::Pos;

/// 
/// Holds a position and a character.
/// 
/// Used for everything, basically, since all things in order to
/// be rendered need 1) a place to be rendered on the screen
/// and 2) a character to represent them on the screen.
///
/// * `pos` - [`Pos`](struct.Pos.html)
/// * `glyph` - Character to represent entity on screen
/// * `fg` - Tcod Color struct representing the foreground color
/// * `bg` - Tcod Color struct representing the background color
/// 
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Entity {
  pub pos: Pos,
  pub glyph: char,
  pub fg: Color,
  pub bg: Color,
}

impl Entity {

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
  /// Move the `Entity` by adding a new [`Pos`](struct.Pos.html) to it
  /// 
  /// This does not overwrite the positon, only add to it.
  /// If values in [`Pos`](struct.Pos.html) are negative, 
  /// this will then just subtract the appropriate values.
  /// 
  /// * `pos` - [`Pos`](struct.Pos.html) struct of ammount to
  /// move in both x and y directions 
  ///  
  pub fn move_pos(&mut self, pos: Pos) {
    self.pos = self.pos + pos;
  }

  ///
  /// Return a new `Entity`
  /// 
  pub fn new(pos: Pos, glyph: char, fg: Color, bg: Color) -> Entity {
    return Entity {
      pos: pos, 
      glyph: glyph, 
      fg: fg, 
      bg: bg
    };
  }

  pub fn set_char(&mut self, glyph: char) {
    self.glyph = glyph;
  }

  pub fn set_pos(&mut self, pos: Pos) {
    self.pos = pos
  }

}
