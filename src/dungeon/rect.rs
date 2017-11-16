use object::Pos;

///
/// Rectangle struct to represent rooms for `Dungeon`
/// 
/// A `Rect` holds the x, y location of it's bottom left
/// corner, and how large it is in terms of width and height.
/// 
/// * `x` - x coordinate
/// * `y` - y coordinate
/// * `l` - Length of room (y-axis)
/// * `w` - Width of room (x-axis)
/// 
/// Note: Has no way of determining by itself whether or not it is out of bounds
/// of the map dimensions, this is up to `Dungeon` to figure out.
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Rect {
  pub x: i32,
  pub y: i32,
  pub l: i32,
  pub w: i32,
}

impl Rect {

  ///
  /// Find the center of the `Rect` and return it's `Pos`ition
  /// 
  /// # Examples
  /// 
  /// ```
  /// let r = Rect::new(10, 5, 20, 20);
  /// let pos = r.center();
  /// assert_eq!(pos, Pos::new(20, 15));
  /// ```
  /// 
  pub fn center(&self) -> Pos {
    return Pos::new(
      (self.w / 2) + self.x, 
      (self.l / 2) + self.y
    );
  }

  ///
  /// Return a new `Rect`
  /// 
  /// # Examples
  /// 
  /// ```
  /// let r = Rect::new(10, 5, 20, 20);
  /// assert_eq!(r.x, 10);
  /// assert_eq!(r.y, 5);
  /// assert_eq!(r.l, 20);
  /// assert_eq!(r.w, 20);
  /// ```
  /// 
  pub fn new(x: i32, y: i32, l: i32, w: i32) -> Rect {
    return Rect { x: x, y: y, l: l, w: w};
  }
  
}

// Test `new()` for `Rect`
#[test]
fn test_rect() {
  let r = Rect::new(10, 5, 20, 20);
  let pos = r.center();
  assert_eq!(pos, Pos::new(20, 15));
}

// Test `center()` for `Rect`
#[test]
fn test_rect_center() {
  let r = Rect::new(10, 5, 20, 20);
  let pos = r.center();
  assert_eq!(pos, Pos::new(20, 15));
}