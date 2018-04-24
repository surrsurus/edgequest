extern crate rand;
use self::rand::{thread_rng, Rng};

use core::world::dungeon::automata::Automaton;

use core::world::dungeon::map::{Grid, Tile};

///
/// Struct to hold the implementation details for the Drunkards' Walk cellular automaton
/// 
/// * `chaos` - Chaos chance from [0.0, 1.0]. Represents the chance that the automaton changes it's direction. 
/// 1.0 represents total chaos, 0.0 represents total order. 
/// 
#[derive(Clone, PartialEq, Debug, Default)]
pub struct DrunkardsWalk {
  pub chaos: f32
}

impl DrunkardsWalk {

  ///
  /// Return a new `DrunkardsWalk`
  /// 
  pub fn new(chaos: f32) -> DrunkardsWalk {
    assert!(chaos >= 0.0 && chaos <= 1.0);
    DrunkardsWalk { chaos: chaos }
  }

}

impl Automaton for DrunkardsWalk {

  type Output = Tile;

  fn generate(&self, grid: &mut Grid<Tile>, sx: Option<usize>, sy: Option<usize>, find: Option<Tile>, replace: Tile, iterations: u32) -> Grid<Tile> {

    // Start our RNG
    let mut rng = thread_rng();

    // Get our starting x and y

    // If x exists, use that x to start
    // Otherwise, randomly generate one
    let mut x : usize;
    match sx {
      Some(sx) => x = sx,
      // Use the length of the outside vectors to determine length
      None => x = rng.gen_range(1, grid.len() - 2)
    }

    // If y exists, use that y to start
    // Otherwise, randomly generate one
    let mut y : usize;
    match sy {
      Some(sy) => y = sy,
      // Use the length of one of the inside vectors to determine length
      None => y = rng.gen_range(1, grid[0].len() - 2)
    }
    
    // Store old dice positions
    let mut old_dice : i32 = rng.gen_range(1, 5);

    for _ in 0..iterations {

      let dice : i32;

      // Generate chaos
      let chaos = rng.gen::<f32>();

      // Determine order/chaos
      if chaos > self.chaos {
        // Order; same as last time
        dice = old_dice;
      } else {
        // Chaos; randomize
        dice = rng.gen_range(1, 5);
        old_dice = dice;
      }

      match dice {
        1 => x += 1,
        2 => x -= 1,
        3 => y += 1,
        4 => y -= 1,
        _ => unreachable!("dice machine broke")
      }

      // Check bounds, leave a gap though between the border.
      // Obviously if your grid is a 1x1 this will cause an issue.
      if x < 1 { x = 1; }
      if y < 1 { y = 1; }
      if x >= grid.len() - 2 { x = grid.len() - 2; }
      if y >= grid[0].len() - 2 { y = grid[0].len() - 2; }

      // Determine what to do based on if `find` is present
      match find.clone() {
        Some(find) => {
          if grid[x][y] == find.clone() {
            grid[x][y] = replace.clone();
          }
        },
        None => {
          grid[x][y] = replace.clone();
        }
      }

    }

    // Return a clone of the grid
    return grid.clone();

  }

}

