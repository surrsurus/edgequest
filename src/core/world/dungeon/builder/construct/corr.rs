use core::world::dungeon::map::Pos;

///
/// Struct to represent a corridor
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
  #[inline]
  pub fn new(start: Pos, end: Pos) -> Corr {
    return Corr { start: start, end: end };
  }

}