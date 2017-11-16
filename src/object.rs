//!
//! Hold the fundamental structs, `Entity` and `Pos`
//! 

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

/// 
/// Allow for the addition of two `Pos` structs
/// 
impl Add for Pos {

    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos { x: self.x + other.x, y: self.y + other.y }
    }

}

/// 
/// Allow for the subtraction of two `Pos` structs
/// 
impl Sub for Pos {

    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos { x: self.x - other.x, y: self.y - other.y }
    }

}

/// 
/// Holds a position and a character.
/// 
/// Used for everything, basically, since all things in order to
/// be rendered need 1) a place to be rendered on the screen
/// and 2) a character to represent them on the screen.
///
/// * `pos` - [`Pos`](struct.Pos.html)
/// * `ch` - Character to represent entity on screen
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Entity {
  pub pos: Pos,
  pub ch: char,
}

/// 
/// Implement some important functions for `Entity`.
/// 
impl Entity {

  /// 
  /// Move the `Entity` by `x` in the x direction and `y` in
  /// the y direction.
  /// 
  /// This does not overwrite the positon, only add to it.
  /// `x` and `y` can be positive or negative.
  /// 
  /// * `x` - ammount to move in the x direction
  /// * `y` - ammount to move in the x direction
  /// 
  pub fn move_cart(&mut self, x: i32, y: i32) {
    self.pos = Pos { x: self.pos.x + x, y: self.pos.y + y };
  }

  /// 
  /// Move the `Entity` by adding a new [`Pos`](struct.Pos.html) to it
  /// 
  /// This does not overwrite the positon, only add to it.
  /// If values in [`Pos`](struct.Pos.html) are negative, 
  /// this will then just subtract the appropriate values.
  /// 
  /// * `pos` - [`Pos`](struct.Pos.html) struct of ammount to
  /// move in both x and y directions 
  ///  
  pub fn move_pos(&mut self, pos: Pos) {
    self.pos = self.pos + pos;
  }

  pub fn set_char(&mut self, ch: char) {
    self.ch = ch;
  }

  pub fn set_pos(&mut self, pos: Pos) {
    self.pos = pos
  }

}

// Test creating Pos structs and adding/subtracting them
#[test]
fn test_pos() {

  let a = Pos { x: 4, y: -3 };
  let b = Pos { x: 1, y: 1 };
  assert_eq!(a + b, Pos { x: 5, y: -2 });

  let c = Pos { x: 0, y: 5 };
  let d = Pos { x: -2, y: 1 };
  assert_eq!(c - d, Pos { x: 2, y: 4 });

}

// Trigger an overflow error for the Add impl
#[test]
#[should_panic]
fn test_upper_bound_pos() {
  use std::i32::MAX;
  Pos { x: MAX + 1, y: MAX + 1 };
}

// Trigger an underflow error for the Sub impl
#[test]
#[should_panic]
fn test_lower_bound_pos() {
  use std::i32::MIN;
  Pos { x: MIN - 1, y: MIN - 1 };
}

// Test creating Entities and it's methods
#[test]
fn test_entity() {
  let mut a = Entity {pos: Pos {x: 1, y: 2}, ch: '@'};
  assert_eq!(a.pos, Pos { x: 1, y: 2 });
  assert_eq!(a.ch, '@');

  a.move_cart(2, 1);
  assert_eq!(a.pos, Pos {x: 3, y: 3});

  a.move_cart(-5, -2);
  assert_eq!(a.pos, Pos {x: -2, y: 1});

  a.set_pos(Pos { x: 0, y: 0 });
  assert_eq!(a.pos, Pos { x: 0, y: 0 });

  a.set_char('!');
  assert_eq!(a.ch, '!');
}