//!
//! A `Pos` is an xy coordinate that can do some fancy math on itself
//!

// Operator overloading
use std::ops::{Add, AddAssign, BitXor, Neg, Sub, SubAssign};

/// 
/// Hold an x, y cartesian coordinate. Should be used when explicitly needing to
/// manipulate points, not just store them. Length and width should be stored as tuples, 
/// however a monster should be a pos to calculate distance between it and the player.
/// 
/// `x` - x axis location
/// `y` - y axis location
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Pos {
  pub x: isize,
  pub y: isize,
}

impl Pos {

  ///
  /// Return a new `Pos`
  /// 
  #[inline]
  pub fn new(x: isize, y: isize) -> Self {
    Pos { x, y }
  }

  ///
  /// Return a new `Pos` from usize's
  /// 
  #[inline]
  pub fn from_usize(x: usize, y: usize) -> Self {
    Pos { x: x as isize, y: y as isize }
  }

  ///
  /// Return a `Pos` from a tuple
  /// 
  #[inline]
  pub fn from_tup(pos: (isize, isize)) -> Self {
    Pos { x: pos.0, y: pos.1 }
  }

  ///
  /// Return a new `Pos` from a tuple of usizes
  /// 
  #[inline]
  pub fn from_usize_tup(pos: (usize, usize)) -> Self {
   Pos { x: pos.0 as isize, y: pos.1 as isize }
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
/// Allow for the addition of two `Pos` structs
/// 
impl Add<Pos> for Pos {

  type Output = Pos;

  #[inline]
  fn add(self, other: Pos) -> Pos {
    Pos::new(self.x + other.x, self.y + other.y)
  }

}

/// 
/// Allow for the addition assignment of `Pos`s
/// 
impl AddAssign<Pos> for Pos {

  #[inline]
  fn add_assign(&mut self, other: Pos) {
    self.x = self.x + other.x;
    self.y = self.y + other.y;
  }

}

///
/// Implement distance formula for `Pos` as ^ in order to find distances between `Pos` structs
/// 
impl BitXor<Pos> for Pos {

  type Output = f32;
  
  #[inline]
  fn bitxor(self, other: Pos) -> f32 {
    (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f32).sqrt()
  }

}

///
/// Allow for unary - on `Pos`
/// 
impl Neg for Pos {

  type Output = Pos;

  #[inline]
  fn neg(self) -> Pos {
    Pos::new(-self.x, -self.y)
  }

}

/// 
/// Allow for the subtraction of two `Pos` structs
/// 
impl Sub<Pos> for Pos {

  type Output = Pos;

  #[inline]
  fn sub(self, other: Pos) -> Pos {
    Pos::new(self.x - other.x, self.y - other.y)
  }

}

/// 
/// Allow for the subtraction assignment of `Pos`s
/// 
impl SubAssign<Pos> for Pos {

  #[inline]
  fn sub_assign(&mut self, other: Pos) {
    self.x = self.x - other.x;
    self.y = self.y - other.y;
  }

}