///
/// Hold a 2D vector of `T`s
/// 
/// Primarily used for dungeon generation and cellular automatons
/// The idea of this is to generate your maps onto a `Grid` and then pass it back
/// up as a `collapse()`d grid.
/// 
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Grid<T : Clone>(pub Vec<Vec<T>>);

impl<T : Clone> Grid<T> {

  ///
  /// Go from a 2D vector to a 1D.
  /// 
  pub fn collapse(self) -> Vec<T> {

    let mut collapsed_grid = Vec::<T>::new();

    for row in self.0 {
      for t in row {
        collapsed_grid.push(t);
      }
    }

    return collapsed_grid.clone();

  }

}

/// 
/// Allow for iteration over `Grid`s
/// 
impl<T : Clone> IntoIterator for Grid<T> {
    
  type Item = T;
  type IntoIter = ::std::vec::IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.collapse().into_iter()
  }

}