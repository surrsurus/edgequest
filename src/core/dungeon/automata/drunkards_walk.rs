use core::dungeon::rand::{thread_rng, Rng};
use core::dungeon::automata::CellularAutomata;
use core::dungeon::constructs::Grid;

pub struct DrunkardsWalk {
  pub chaos: i32
}

impl CellularAutomata for DrunkardsWalk {

  type Output = u8;

  fn generate(grid: &mut Grid<u8>, find: u8, replace: u8, iterations: u32) -> Grid<u8> {

    let mut rng = thread_rng();

    let mut x: i32 = rng.gen_range(1, grid.0.len() as i32 - 2);
    let mut y: i32 = rng.gen_range(1, grid.0[0].len() as i32 - 2);

    for _ in 0..iterations {

      let dice = rng.gen_range(1, 5);

      match dice {
        1 => x += 1,
        2 => x -= 1,
        3 => y += 1,
        4 => y -= 1,
        _ => panic!("Literally should never be possible.")
      }

      // Check bounds, leave a gap though between the border.
      // Obviously if your grid is a 1x1 this will cause an issue.
      if x < 1 { x = 1; }
      if y < 1 { y = 1; }
      if x >= grid.0.len() as i32 - 2 { x = grid.0.len() as i32 - 2; }
      if y >= grid.0[0].len() as i32 - 2 { y = grid.0[0].len() as i32 - 2; }

      if grid.0[x as usize][y as usize] == find {
        grid.0[x as usize][y as usize] = replace;
      }

    }

    return grid.clone();

  }

}

