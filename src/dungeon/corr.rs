use object::Pos;

///
/// Struct to represent a corridor for `Dungeon`
/// 
/// All a corridor is is a start location and an end location where the corridor
/// should be. How it gets there is not up to the corridor.
/// 
/// * `start` - Starting position as `Pos`
/// * `end` - Ending position as `Pos`
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Corr {
  pub start: Pos,
  pub end: Pos,
}

impl Corr {

  /// 
  /// Return a new `Corr`
  /// 
  /// # Examples
  /// 
  /// ```
  /// let c = Corr::new(Pos::new(0, 1), Pos::new(1, 2));
  /// assert_eq!(c.start, Pos::new(0, 1));
  /// assert_eq!(c.end, Pos::new(1, 2));
  /// ```
  /// 
  pub fn new(start: Pos, end: Pos) -> Corr {
    return Corr { start: start, end: end };
  }

}

#[test]
fn test_corr() {
  let c = Corr::new(Pos::new(0, 1), Pos::new(1, 2));
  assert_eq!(c.start, Pos::new(0, 1));
  assert_eq!(c.end, Pos::new(1, 2));
}