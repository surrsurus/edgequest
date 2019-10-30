//!
//! Hold colors as a tuple
//! 

// Operator overloading
use std::ops::{Add, AddAssign, Sub, SubAssign};

// Tcod colors for conversion
use core::tcod::colors;

///
/// One would think that the RGB 'type' we use here is divorced from the actual tcod library, however
/// this construct serves more as a wrapper or converter for tcod colors, because they are pretty ass.
/// 
/// Tcod colors cannot be cloned, cannot be compared, and other basic functions a normal human being
/// would assume would be available are simply not. I assume this is not the doings of the rust
/// maintainers, but the original author of libtcod combined with whatever necessary evils and/or evil rituals
/// that must have taken place in order to get tcod-rs working.
/// 
/// While this is specifically intended for tcod, it can be entirely used without ever needing
/// to see tcod as it still holds all of the data on it's own.
/// 
/// Naturally since RGB colors don't exceed values of 255, the RGB struct holds 3 u8 values.
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct RGB (pub u8, pub u8, pub u8);

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
  pub fn to_tup(self) -> (u8, u8, u8) {
    (self.r(), self.g(), self.b())
  }

  /// 
  /// Convert an RGB into a tcod Color
  /// 
  #[inline]
  pub fn to_tcod(self) -> colors::Color {
    colors::Color::new(self.r(), self.g(), self.b())
  }

  #[inline]
  pub fn r(self) -> u8 {
    self.0
  }

  #[inline]
  pub fn g(self) -> u8 {
    self.1
  }

  #[inline]
  pub fn b(self) -> u8 {
    self.2
  }

  ///
  /// Take two RGBs. Compute a color between each of them.
  /// Amount is a float between 0 and 1, and specifies how far
  /// in between the two colors the new color should be.
  ///
  pub fn transition_between(rgb1: &RGB, rgb2: &RGB, amount: f32) -> RGB {
    let diff : (isize, isize, isize) = (
      (rgb2.r() as isize) - (rgb1.r() as isize),
      (rgb2.g() as isize) - (rgb1.g() as isize), 
      (rgb2.b() as isize) - (rgb1.b() as isize)
    );
    let change : (isize, isize, isize) = (
      (diff.0 as f32 * amount).round() as isize,
      (diff.1 as f32 * amount).round() as isize,
      (diff.2 as f32 * amount).round() as isize
    );
    let result : RGB = RGB(
      (rgb1.r() as isize + change.0) as u8,
      (rgb1.g() as isize + change.1) as u8,
      (rgb1.b() as isize + change.2) as u8
    );
    return result;
  }

  ///
  /// Take three RGBs. Compute how far in between 
  /// rgb1 and rgb2 rgb3 is.
  ///
  pub fn transition_distance(rgb1: &RGB, rgb2: &RGB, rgb3 : &RGB) -> f32 {
    // We need to go through each r, g, and b value seperately, as RGB / RGB is not well defined.
    // We also need to be careful to avoid zero values - we do not want divide by zero errors.

    // Starting with red.
    let change_r : isize = (rgb3.r() as isize) - (rgb1.r() as isize);
    let diff_r   : isize = (rgb2.r() as isize) - (rgb1.r() as isize);
    // If the difference in red value between rgb1 and rgb2 is 0,
    // EVERY DISTANCE between those two colors would provide the same result.
    // This is identified mathematically as some number over 0, and is why we
    // need the conditional logic here.
    if diff_r != 0 {
      let amount : f32 = (change_r as f32) / (diff_r as f32);
      return amount;
    }

    // Green.
    let change_g : isize = (rgb3.g() as isize) - (rgb1.g() as isize);
    let diff_g   : isize = (rgb2.g() as isize) - (rgb1.g() as isize);
    if diff_g != 0 {
      let amount : f32 = (change_g as f32) / (diff_g as f32);
      return amount;
    }

    // Blue.
    let change_b : isize = (rgb3.b() as isize) - (rgb1.b() as isize);
    let diff_b   : isize = (rgb2.b() as isize) - (rgb1.b() as isize);
    if diff_b != 0 {
      let amount : f32 = (change_b as f32) / (diff_b as f32);
      return amount;
    }
    // The rgb1 and rgb2 values were the same. Just return zero.
    return 0.0
  }
}

/// 
/// Allow for the addition of two `RGB` structs
/// 
impl Add<RGB> for RGB {

  type Output = RGB;

  #[inline]
  fn add(self, other: RGB) -> RGB {
    let r = { if (self.r() as isize) + (other.0 as isize) > 255 { 255 } else { self.r() + other.0 } };
    let g = { if (self.g() as isize) + (other.1 as isize) > 255 { 255 } else { self.g() + other.1 } };
    let b = { if (self.b() as isize) + (other.2 as isize) > 255 { 255 } else { self.b() + other.2 } };
    return RGB(r, g, b);
  }

}

impl AddAssign<RGB> for RGB {
  
  #[inline]
  fn add_assign(&mut self, other: Self) {
    if (self.r() as isize) + (other.0 as isize) > 255 { self.0 = 255; } else { self.0 += other.0 }
    if (self.g() as isize) + (other.1 as isize) > 255 { self.1 = 255; } else { self.1  += other.1 }
    if (self.b() as isize) + (other.2 as isize) > 255 { self.2 = 255; } else { self.2 += other.2 }
  }

}

/// 
/// Allow for the addition of two `RGB` structs
/// 
impl Sub<RGB> for RGB {

  type Output = RGB;

  #[inline]
  fn sub(self, other: RGB) -> RGB {
    let r = { if (self.r() as isize) - (other.0 as isize) < 0 { 0 } else { self.r() - other.0 } };
    let g = { if (self.g() as isize) - (other.1 as isize) < 0 { 0 } else { self.g() - other.1 } };
    let b = { if (self.b() as isize) - (other.2 as isize) < 0 { 0 } else { self.b() - other.2 } };
    return RGB(r, g, b);
  }

}

impl SubAssign<RGB> for RGB {
  
  #[inline]
  fn sub_assign(&mut self, other: Self) {
    if (self.r() as isize) - (other.0 as isize) < 0 { self.0 = 0; } else { self.0 -= other.0 }
    if (self.g() as isize) - (other.1 as isize) < 0 { self.1 = 0; } else { self.1 -= other.1 }
    if (self.b() as isize) - (other.2 as isize) < 0 { self.2 = 0; } else { self.2 -= other.2 }
  }

}