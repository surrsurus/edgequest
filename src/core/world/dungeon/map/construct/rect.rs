use core::world::dungeon::map::Pos;

///
/// Rectangle struct to represent rooms for `Dungeon`
/// 
/// A `Rect` holds the x, y location of it's bottom left
/// corner, and how large it is in terms of width and height.
/// 
/// * `x` - x coordinate
/// * `y` - y coordinate
/// * `h` - Height of room (y-axis)
/// * `w` - Width of room (x-axis)
/// 
/// Note: Has no way of determining by itself whether or not it is out of bounds
/// of the map dimensions, this is up to `Dungeon` to figure out.
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Rect {
  pub x: isize,
  pub y: isize,
  pub h: isize,
  pub w: isize,
}

impl Rect {

  ///
  /// Find the center of the `Rect` and return it's position
  /// 
  #[inline]
  pub fn center(&self) -> Pos {
    Pos::from_tup((
      (self.w / 2) + self.x, 
      (self.h / 2) + self.y
    ))
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
  /// assert_eq!(r.h, 20);
  /// assert_eq!(r.w, 20);
  /// ```
  /// 
  #[inline]
  pub fn new(x: isize, y: isize, h: isize, w: isize) -> Self {
    Rect { x, y, h, w }
  }
  
}