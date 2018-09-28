// Operator overloading
use std::ops::{Add, Sub};

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
  /// Get an RGB to a tuple
  /// 
  #[inline]
  pub fn to_tup(rgb: RGB) -> (u8, u8, u8) {
    (rgb.0, rgb.1, rgb.2)
  }

  /// 
  /// Convert RGB to a tcod Color
  /// 
  #[inline]
  pub fn to_tcod(&self) -> colors::Color {
    colors::Color::new(self.0, self.1, self.2)
  }

}

/// 
/// Allow for the addition of two `RGB` structs
/// 
impl Add<RGB> for RGB {

  type Output = RGB;

  #[inline]
  fn add(self, other: RGB) -> RGB {
    let r = { if (self.0 as isize) + (other.0 as isize) > 255 { 255 } else { self.0 + other.0 } };
    let g = { if (self.1 as isize) + (other.1 as isize) > 255 { 255 } else { self.1 + other.1 } };
    let b = { if (self.2 as isize) + (other.2 as isize) > 255 { 255 } else { self.2 + other.2 } };
    return RGB(r, g, b);
  }

}

/// 
/// Allow for the addition of two `RGB` structs
/// 
impl Sub<RGB> for RGB {

  type Output = RGB;

  #[inline]
  fn sub(self, other: RGB) -> RGB {
    let r = { if (self.0 as isize) - (other.0 as isize) < 0 { 0 } else { self.0 - other.0 } };
    let g = { if (self.1 as isize) - (other.1 as isize) < 0 { 0 } else { self.1 - other.1 } };
    let b = { if (self.2 as isize) - (other.2 as isize) < 0 { 0 } else { self.2 - other.2 } };
    return RGB(r, g, b);
  }

}