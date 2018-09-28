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
  /// Return a `Pos` from a tuple
  /// 
  #[inline]
  pub fn from_tup(pos: (isize, isize)) -> Pos {
    Pos { x: pos.0, y: pos.1 }
  }

  ///
  /// Return a tuple from a `Pos`
  /// 
  #[inline]
  pub fn as_tup(&self) -> (isize, isize) {
    (self.x, self.y)
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
/// Allow for the addition of a `Pos` and a tuple
/// 
impl Add<(isize, isize)> for Pos {

  type Output = Pos;

  #[inline]
  fn add(self, other: (isize, isize)) -> Pos {
    Pos::new(self.x + other.0, self.y + other.1)
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
/// Allow for the addition assignment of `Pos` with a tuple
/// 
impl AddAssign<(isize, isize)> for Pos {

  #[inline]
  fn add_assign(&mut self, other: (isize, isize)) {
    self.x = self.x + other.0;
    self.y = self.y + other.1;
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
/// Implement distance formula for `Pos` and a tuple as ^ in order to find distances between them
/// 
impl BitXor<(isize, isize)> for Pos {

  type Output = f32;
  
  #[inline]
  fn bitxor(self, other: (isize, isize)) -> f32 {
    (((other.0 - self.x).pow(2) + (other.1 - self.y).pow(2)) as f32).sqrt()
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
/// Allow for the subtraction of a `Pos` with a tuple
/// 
impl Sub<(isize, isize)> for Pos {

  type Output = Pos;

  #[inline]
  fn sub(self, other: (isize, isize)) -> Pos {
    Pos::new(self.x - other.0, self.y - other.1)
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

/// 
/// Allow for the subtraction assignment of `Pos` with a tuple
/// 
impl SubAssign<(isize, isize)> for Pos {

  #[inline]
  fn sub_assign(&mut self, other: (isize, isize)) {
    self.x = self.x - other.0;
    self.y = self.y - other.1;
  }

}