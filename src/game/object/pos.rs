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
  pub x: i32,
  pub y: i32,
}

impl Pos {

  ///
  /// Return a new `Pos`
  /// 
  pub fn new(x: i32, y: i32) -> Pos {
    return Pos { x: x, y: y};
  }

  ///
  /// Return a `Pos` at the origin (0, 0)
  /// 
  pub fn origin() -> Pos {
    return Pos { x: 0, y: 0 };
  }

}

///
/// Implement distance formula for `Pos` as ^ in order to find distances between `Pos` structs
/// 
impl BitXor for Pos {

  type Output = f32;
  fn bitxor(self, other: Pos) -> f32 {
    return (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f32).sqrt();
  }

}


/// 
/// Allow for the addition of two `Pos` structs
/// 
impl Add for Pos {

    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
      return Pos::new(self.x + other.x, self.y + other.y);
    }

}

/// 
/// Allow for the subtraction of two `Pos` structs
/// 
impl Sub for Pos {

    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
      return Pos::new(self.x - other.x, self.y - other.y);
    }

}