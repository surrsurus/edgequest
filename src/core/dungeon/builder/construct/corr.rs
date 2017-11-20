///
/// Struct to represent a corridor for `Dungeon`
/// 
/// All a corridor is is a start location and an end location where the corridor
/// should be. How it gets there is not up to the corridor.
/// 
/// * `start` - Starting position as `(usize, usize)`
/// * `end` - Ending position as `(usize, usize)`
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Corr {
  pub start: (usize, usize),
  pub end: (usize, usize),
}

impl Corr {

  /// 
  /// Return a new `Corr`
  /// 
  #[inline]
  pub fn new(start: (usize, usize), end: (usize, usize)) -> Corr {
    return Corr { start: start, end: end };
  }

}