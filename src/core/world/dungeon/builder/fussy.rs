extern crate fuss;
use self::fuss::Simplex;

use core::world::dungeon::builder::Buildable;
use core::world::dungeon::map::Grid;

///
/// Builder for generating noise maps
/// 
pub struct Fussy {
  pub grid: Grid<u8>,
  pub w: usize,
  pub h: usize,
  pub noise: Simplex,
  pub threshold: f32
}

impl Fussy {

  ///
  /// Add noise to the map based on sum_octaves 
  /// 
  /// Will only "fill" tile if the threshold is met, threshold should be anywhere between [0, 2]
  /// 
  fn add_noise(&mut self) {
    for x in 0..self.w {
      for y in 0..self.h {
        if self.noise.sum_octave_2d(16, x as f32, y as f32, 0.5, 0.007) + 1.0 > self.threshold {
          self.grid[x][y] = 1;
        }
      }
    }
  }

  ///
  /// Return a new `Fussy`
  /// 
  pub fn new(grid: Grid<u8>, threshold: f32) -> Fussy {

    // Make a new dungeon with our fresh grid of size `w` by `h`
    let f = Fussy { 
      grid: grid.clone(), 
      w: grid.len(), 
      h: grid[0].len(),
      noise: Simplex::new(),
      threshold: threshold
    };

    return f;

  }

}

impl Buildable for Fussy {
  
  type Output = u8;

  fn build(&mut self) -> Grid<u8> {
    self.add_noise();
    return self.grid.clone();
  }

}