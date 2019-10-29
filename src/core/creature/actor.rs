//!
//! An `Actor` is an object that is renderable to the screen and is responsible for moving itself
//!

use core::world::dungeon::map::Pos;
use core::renderer::{Renderable, RGB};

///
/// Actor struct. Holds necessary properties that extend from `Renderable`
///
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Actor {
  name: &'static str,
  glyph: char,
  pub pos: Pos,
  pub prev_pos: Pos,
  fg: RGB,
  bg: RGB
}

impl Actor {

  /// 
  /// Move the `Actor` by `x` in the x direction and `y` in
  /// the y direction.
  /// 
  /// This does not overwrite the position, only add to it.
  /// `x` and `y` can be positive or negative.
  /// 
  /// * `x` - amount to move in the x direction
  /// * `y` - amount to move in the y direction
  /// 
  #[inline]
  pub fn move_cart(&mut self, x: isize, y: isize) {
    self.pos = Pos::new(self.pos.x + x, self.pos.y + y);
  }

  /// 
  /// Move the `Actor` by adding a new `Pos` to it
  /// 
  /// This does not overwrite the positron, only add to it.
  /// If values in `Pos` are negative, 
  /// this will then just subtract the appropriate values.
  /// 
  /// * `pos` - `Pos` struct of amount to
  /// move in both x and y directions 
  ///  
  #[inline]
  pub fn move_pos(&mut self, pos: Pos) {
    self.pos += pos;
  }

  ///
  /// Return a new `Actor`
  ///
  #[inline]
  pub fn new(name: &'static str, glyph: char, pos: Pos, fg: RGB, bg: RGB) -> Self {
    Actor {
      name: name,
      glyph: glyph, 
      pos: pos,
      prev_pos: pos,
      fg: fg, 
      bg: bg
    }
  }

  ///
  /// Directly override position
  ///
  #[inline]
  pub fn set_pos(&mut self, pos: Pos) {
    self.pos = pos
  }

}

///
/// Implement the `Renderable` trait for `Actor`, mostly just getters and setters
///
impl Renderable for Actor {

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
  fn get_id(&self) -> &'static str {
    self.name.clone()
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

  #[inline]
  fn set_id(&mut self, name: &'static str) {
    self.name = name;
  }

}