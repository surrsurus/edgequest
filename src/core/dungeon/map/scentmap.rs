use core::dungeon::map::{Grid, Tile};

///
/// What value the player sets the scent of nearby tiles to
/// 
const INC : u8 = 150;

///
/// Affects distance that bloom around player travels
/// 
const BLOOM : f32 = 0.05; 

///
/// Decay value applied to tiles inheriting scent from neighbors
/// 
const DECAY : f32 = (255.0/256.0);

///
/// Scent struct. Holds values for the ScentMap, used by monsters to track the player.
/// 
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Scent {
  pub value: u8,
}

///
/// Keep track of scent values
/// 
impl Scent {

  ///
  /// Return a new `Scent`
  /// 
  pub fn new() -> Scent {
    Scent { value: 0 }
  }

  ///
  /// Return `Scent` as float
  /// 
  pub fn as_f32(&self) -> f32 {
    self.value as f32
  }

  ///
  /// Increment `Scent` by INC
  /// 
  pub fn inc(&mut self) {
    if self.value < 255 - INC {
      self.value = INC;
    }
  }

  ///
  /// Filter a `Scent`. 1 if value > 0, 0.1 otherwise
  /// 
  /// The 0.1 prevents a divide by zero error when calling `avg_of_neighbors()`
  /// 
  pub fn filter(&self) -> f32 {
    if self.value == 0 {
      0.1
    } else {
      1.0
    }
  }
  
}

///
/// `Grid` of `Scent`s
/// 
pub type ScentMap = Grid<Scent>;

impl ScentMap {

  ///
  /// Return an f32 value that is the average value of `Scent`s surrounding the desired position, with a slight decay factor  
  /// 
  /// This is not a "true" average of all neighboring `Scent`s.
  /// 
  fn avg_of_neighbors(&self, buffer: &ScentMap, x: usize, y: usize) -> f32 {

    (
      
      // Add all tile values
      (buffer.0[x - 1][y].as_f32() +
      buffer.0[x + 1][y].as_f32() +
      buffer.0[x][y - 1].as_f32() +
      buffer.0[x][y + 1].as_f32() +
      buffer.0[x + 1][y + 1].as_f32() +
      buffer.0[x - 1][y - 1].as_f32() +
      buffer.0[x + 1][y - 1].as_f32() +
      buffer.0[x - 1][y + 1].as_f32()) / 
      
      // Divide by num tiles present, to get the average
      // Add a little bit more on top to make the bloom around player larger
      (((buffer.0[x - 1][y].filter() +
      buffer.0[x + 1][y].filter() +
      buffer.0[x][y - 1].filter() +
      buffer.0[x][y + 1].filter() +
      buffer.0[x + 1][y + 1].filter() +
      buffer.0[x - 1][y - 1].filter() +
      buffer.0[x + 1][y - 1].filter() +
      buffer.0[x - 1][y + 1].filter()) + BLOOM) 
      
      // Decay factor
      * DECAY)

    ) 

  }
  
  ///
  /// Update the `ScentMap`
  /// 
  pub fn update(&mut self, grid: &Grid<Tile>, player_pos: (isize, isize)) {

    let width : usize = self.0.len();
    let height : usize = self.0[0].len();

    // Define a closure to make testing valid scent tiles easier
    let is_valid = |x: usize, y: usize| -> bool {
      // Test first to optimize since if a tile can't have objects pass through it
      // there is no need to have scent permeate it
      if !grid.0[x][y].blocks {
        x > 1 && x + 1 < width && y > 1 && y + 1 < height
      } else {
        false
      }
    };

    // Create initial bloom around player
    for nx in -1..2 {
      for ny in -1..2 {
        if is_valid((player_pos.0 - nx) as usize , (player_pos.1 - ny) as usize) {
          self.0[(player_pos.0 - nx) as usize][(player_pos.1 - ny) as usize].inc();
        }
      }
    }

    // Create buffer
    let buffer = self.clone();

    // Change values of map based on averages from the buffer
    for x in 0..width {
      for y in 0..height {
        if is_valid(x, y) {
          self.0[x][y].value = self.avg_of_neighbors(&buffer, x, y) as u8;
        }
      }
    }

  }

}