use core::object::{Pos, Entity, RenderableEntity, RGB, Renderable};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Fighter {
  name: String,
  glyph: char,
  pub pos: Pos,
  fg: RGB,
  bg: RGB
}

impl Fighter {

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
  #[inline]
  pub fn move_cart(&mut self, x: isize, y: isize) {
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
  #[inline]
  pub fn move_pos(&mut self, pos: Pos) {
    self.pos = self.pos + pos;
  }

  ///
  /// Return a new `Entity`
  ///
  #[inline]
  pub fn new(name: String, glyph: char, pos: Pos, fg: RGB, bg: RGB) -> Fighter {
    Fighter {
      name: name,
      glyph: glyph, 
      pos: pos, 
      fg: fg, 
      bg: bg
    }
  }

  #[inline]
  pub fn set_pos(&mut self, pos: Pos) {
    self.pos = pos
  }

}

impl Entity for Fighter {

  #[inline]
  fn get_name(&self) -> String {
    self.name.clone()
  }

  #[inline]
  fn set_name(&mut self, name: String) {
    self.name = name;
  }

}

impl Renderable for Fighter {

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

impl RenderableEntity for Fighter {}