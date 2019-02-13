use core::world::dungeon::map;

pub mod structure;
pub use self::structure::Structure;

pub mod simple;
pub use self::simple::Simple;

///
/// `Filter` trait to define a uniform set of behavior for dungeon generation
/// 
pub trait Filter {

  /// 
  /// What type `T` of `Grid<T>` is on output
  /// 
  type Output : Clone;

  fn apply(&mut self, grid: &mut map::Grid<Self::Output>);

}