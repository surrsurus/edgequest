use core::object::{Entity, Pos, RGB};

///
/// A Fighter is something that can fight, but also be rendered to the screen
///
/// NOTE: From that description, I think something might have to change...
///
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Fighter {
  name: &'static str,
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
  /// * `y` - ammount to move in the y direction
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
  pub fn new(name: &'static str, glyph: char, pos: (isize, isize), fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Fighter {
    Fighter {
      name: name,
      glyph: glyph, 
      pos: Pos::from_tup(pos), 
      fg: RGB::from_tup(fg), 
      bg: RGB::from_tup(bg)
    }
  }

  #[inline]
  pub fn set_pos(&mut self, pos: Pos) {
    self.pos = pos
  }

}

impl Entity for Fighter {

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