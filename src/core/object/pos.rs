// Operator overloading
use std::ops::{Add, Sub, BitXor};

/// 
/// Hold an x, y cartesian coordinate
/// 
/// `x` - x axis location
/// `y` - y axis location
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Pos {
  // i32 because of tcod
  pub x: isize,
  pub y: isize,
}

impl Pos {

  ///
  /// Return a new `Pos`
  /// 
  #[inline]
  pub fn new(x: isize, y: isize) -> Pos {
    Pos { x: x, y: y }
  }

  ///
  /// Return a new `Pos` from usize's
  /// 
  #[inline]
  pub fn from_usize(x: usize, y: usize) -> Pos {
    Pos { x: x as isize, y: y as isize }
  }

  ///
  /// Return a `Pos` at the origin (0, 0)
  /// 
  #[inline]
  pub fn origin() -> Pos {
    Pos { x: 0, y: 0 }
  }

}

///
/// Implement distance formula for `Pos` as ^ in order to find distances between `Pos` structs
/// 
impl BitXor for Pos {

  type Output = f32;
  
  fn bitxor(self, other: Pos) -> f32 {
    (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f32).sqrt()
  }

}


/// 
/// Allow for the addition of two `Pos` structs
/// 
impl Add for Pos {

    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
      Pos::new(self.x + other.x, self.y + other.y)
    }

}

/// 
/// Allow for the subtraction of two `Pos` structs
/// 
impl Sub for Pos {

    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
      Pos::new(self.x - other.x, self.y - other.y)
    }

}