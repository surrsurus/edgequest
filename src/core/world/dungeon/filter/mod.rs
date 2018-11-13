use core::world::dungeon::map;

///
/// `Automaton` trait to define a uniform set of behavior for dungeon generation
/// 
pub trait Filter {

  /// 
  /// What type `T` of `Grid<T>` is on output
  /// 
  type Output : Clone;

  fn apply(grid: &map::Grid<Self::Output>) -> map::Grid<Self::Output>;

}