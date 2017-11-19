pub trait CellularAutomata {

  fn generate(grid: &mut Vec<Vec<u8>>, find: u8, replace: u8, iterations: u32) -> Vec<Vec<u8>>;

}