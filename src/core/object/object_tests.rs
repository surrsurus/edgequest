#[cfg(test)]
mod tests {
  use core::tcod::colors::Color;

  use core::object::{Entity, Floor, Pos, RGB, Tile};

  // Test creating Pos structs and adding/subtracting them
  #[test]
  fn test_pos() {

    let a = Pos::new(4, -3);
    let b = Pos::new(1, 1);
    assert_eq!(a + b, Pos::new(5, -2));

    let c = Pos { x: 0, y: 5 };
    let d = Pos { x: -2, y: 1 };
    assert_eq!(c - d, Pos::new(2, 4));

  }

  // Trigger an overflow error for the Add impl
  #[test]
  #[should_panic]
  fn test_upper_bound_pos() {
    use std::i32::MAX;
    Pos::new(MAX + 1, MAX + 1);
  }

  // Trigger an underflow error for the Sub impl
  #[test]
  #[should_panic]
  fn test_lower_bound_pos() {
    use std::i32::MIN;
    Pos::new(MIN - 1, MIN - 1);
  }


  // Test creating Entities and it's methods
  #[test]
  fn test_entity() {

    let mut a = Entity::new(Pos::new(1, 2), '@', RGB(255, 255, 255), RGB(0, 0, 0));
    assert_eq!(a.pos, Pos::new(1, 2));
    assert_eq!(a.glyph, '@');

    a.move_cart(2, 1);
    assert_eq!(a.pos, Pos::new(3, 3));

    a.move_cart(-5, -2);
    assert_eq!(a.pos, Pos::new(-2, 1));

    a.set_pos(Pos { x: 0, y: 0 });
    assert_eq!(a.pos, Pos::new(0, 0));

    a.set_char('!');
    assert_eq!(a.glyph, '!');

  }

}
