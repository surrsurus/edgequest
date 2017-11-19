
#[cfg(test)]
mod tests {
  use core::dungeon::{Corr, Rect};
  
  #[test]
  fn test_corr() {
    let c = Corr::new((0, 1), (1, 2));
    assert_eq!(c.start, (0, 1));
    assert_eq!(c.end, (1, 2));
  }

  // Test `new()` for `Rect`
  #[test]
  fn test_rect() {
    let r = Rect::new(10, 5, 20, 20);
    let pos = r.center();
    assert_eq!(pos, (20, 15));
  }

  // Test `center()` for `Rect`
  #[test]
  fn test_rect_center() {
    let r = Rect::new(10, 5, 20, 20);
    let pos = r.center();
    assert_eq!(pos, (20, 15));
  }

}