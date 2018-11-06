//!
//! Hold colors as a tuple
//! 

// Operator overloading
use std::ops::{Add, Sub};

// Tcod colors for conversion
use core::tcod::colors;

///
/// One would think that the RGB 'type' we use here is divorced from the actual tcod library, however
/// this construct serves more as a wrapper or converter for tcod colors, because they are pretty ass.
/// 
/// Tcod colors cannot be cloned, cannot be compared, and other basic functions a normal human being
/// would assume would be availible are simply not. I assume this is not the doings of the rust
/// maintainers, but the original author of libtcod combined with whatever necessary evils and/or evil rituals
/// that must have taken place in order to get tcod-rs working.
/// 
/// While this is specifically intended for tcod, it can be entirely used without ever needing
/// to see tcod as it still holds all of the data on it's own.
/// 
/// Naturally since RGB colors don't exceed values of 255, the RGB struct holds 3 u8 values.
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
  /// Turn an RGB into a tuple
  /// 
  #[inline]
  pub fn to_tup(rgb: RGB) -> (u8, u8, u8) {
    (rgb.0, rgb.1, rgb.2)
  }

  /// 
  /// Convert an RGB into a tcod Color
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