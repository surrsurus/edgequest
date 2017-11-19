#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Grid<T> {

  pub fn collapse(self) -> Vec<T> {

    let mut collapsed_grid = Vec::<T>::new();

    for row in self.0 {
      for t in row {
        collapsed_grid.push(t);
      }
    }

    return collapsed_grid;

  }

}

impl<T> IntoIterator for Grid<T> {
    
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
      self.collapse().into_iter()
    }

}