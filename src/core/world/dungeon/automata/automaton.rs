use core::world::dungeon::map;

///
/// `Automaton` trait to define a set of behavior for all cellular automatons
/// 
pub trait Automaton {

  /// 
  /// What type `T` of `Grid<T>` is on output and input
  /// 
  type Output : Clone;

  ///
  /// Generate the cellular automata
  /// 
  /// We take some `Grid<T>` of undefined length and width where `T` is set to `Self::Output` and
  /// operate the cellular automaton over the grid, replacing instances of `find` with
  /// `replace`. If `find` is `None`, the automaton should replace every `T` it finds with
  /// `replace`.
  /// 
  /// This is designed so that cellular automatons can operate sequentially on a grid, constantly adding to it.
  /// This is typically intended to work with `Tile`s, however it can work with any type `T`
  /// 
  /// It is implied that you should initialize each builder with it's `new()` method
  /// then call this function
  /// 
  fn generate(&self, grid: &mut map::Grid<Self::Output>, x: Option<usize>, y: Option<usize>, find: Option<Self::Output>, replace: Self::Output, iterations: u32) -> map::Grid<Self::Output>;

}