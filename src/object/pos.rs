// Operator overloading
use std::ops::{Add, Sub};

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