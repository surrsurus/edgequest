//!
//! Drunkard's Walk
//! 
//! A super basic cellular automata
//! 

use super::Automaton;
use core::world::dungeon::map::{self, Pos, Tile};

///
/// Struct to hold the implementation details for the Drunkards' Walk cellular automaton
/// 
/// * `chaos` - Chaos chance from [0.0, 1.0]. Represents the chance that the automaton changes it's direction. 
/// 1.0 represents total chaos, 0.0 represents total order. Going over 1.0 or under 0.0 causes a panic.
/// 
#[derive(Clone, PartialEq, Debug, Default)]
pub struct DrunkardsWalk {
  pub chaos: f32
}

impl DrunkardsWalk {

  ///
  /// Return a new `DrunkardsWalk`
  /// 
  /// Will panic if chaos is not between the values of [0.0, 1.0] inclusive.
  /// 
  pub fn new(chaos: f32) -> Self {
    assert!(chaos >= 0.0 && chaos <= 1.0);
    DrunkardsWalk { chaos }
  }

}

impl Automaton for DrunkardsWalk {

  type Output = Tile;

  fn apply(&self, grid: &mut map::Grid<Tile>, pos: Option<Pos>, find: Option<Tile>, replace: Tile, iterations: u32) {

    // Get our starting x and y

    let mut starting_pos = self.unwrap_pos(grid, pos);
    
    // Store old dice positions. Initialize with whatever
    let mut old_dice = self.get_d4();

    for _ in 0..iterations {

      let dice : usize;

      // Generate chaos
      let chaos = self.get_chaos();

      // Determine order/chaos
      if chaos > self.chaos {
        // Order; same as last time
        dice = old_dice;
      } else {
        // Chaos; randomize
        dice = self.get_d4();
        old_dice = dice;
      }

      match dice {
        1 => starting_pos.x += 1,
        2 => starting_pos.x -= 1,
        3 => starting_pos.y += 1,
        4 => starting_pos.y -= 1,
        _ => unreachable!("DrunkardsWalk - Unreachable dice state reached in movement")
      }

      // Place pos inbounds
      self.place_inbounds(grid, &mut starting_pos);

      // Determine what to do based on if `find` is present
      match find.clone() {
        Some(find) => {
          if grid[starting_pos] == find.clone() {
            grid[starting_pos] = replace.clone();
          }
        },
        None => {
          grid[starting_pos] = replace.clone();
        }
      }

    }

  }

}

