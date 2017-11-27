use core::tcod::colors;

///
/// Wrap tcod::Color with a struct to keep tcod integration with the renderer
/// 
/// Also allows us to be able to clone and compare, where tcod colors cannot for some reason
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct RGB(pub u8, pub u8, pub u8);

impl RGB {

  ///
  /// Get an RGB from a tuple
  /// 
  #[inline]
  pub fn from_tup(rgb: (u8, u8, u8)) -> RGB {
    RGB(rgb.0, rgb.1, rgb.2)
  }

  /// 
  /// Convert RGB to a tcod Color
  /// 
  #[inline]
  pub fn to_tcod(&self) -> colors::Color {
    colors::Color::new(self.0, self.1, self.2)
  }

}