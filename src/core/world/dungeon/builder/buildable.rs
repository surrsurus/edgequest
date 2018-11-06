use core::world::dungeon::map;

///
/// Buildable trait to define a set of behaviors that all dungeon builders will inherit
/// 
pub trait Buildable {

  /// 
  /// What type `T` of `Grid<T>` is on output
  /// 
  type Output : Clone;

  ///
  /// Build the dungeon and return a `Grid<Self::Output>`
  /// 
  /// It is implied that you should initialize each builder with it's `new()` method
  /// then call this function
  /// 
  fn build(&mut self) -> map::Grid<Self::Output>;

}