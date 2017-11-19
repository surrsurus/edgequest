use core::dungeon::Grid;

pub trait CellularAutomata {

  type Output;

  fn generate(grid: &mut Grid<Self::Output>, find: Self::Output, replace: Self::Output, iterations: u32) -> Grid<Self::Output>;

}