extern crate rand;
use self::rand::{thread_rng, Rng};

use std::io::prelude::*;
use std::fs::{self};

use core::world::dungeon::builder::Buildable;

use core::world::dungeon::map::{FloorType, Grid, Tile, TileType, WallType};

///
/// Simple dungeon builder
///
/// This builder places a number of small rooms (respective to map size)
/// all connected by corridors.
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Structure {
  grid: Grid<Tile>,
  w: usize,
  h: usize,
}

impl Structure {

  ///
  /// Add a random structure
  ///
  fn add_rand_struct(&mut self) {

    // Okay. Hang on

    // RNG
    let mut rng = rand::thread_rng();

    // Create a vector out of collecting the read_dir by mapping the unwrapped paths
    let paths : Vec<_> = fs::read_dir("./strct").unwrap().map(|res| res.unwrap().path()).collect();

    // Choose a random element 
    let mut file = fs::File::open(rng.choose(&paths).unwrap()).unwrap();

    // Create empty string
    let mut s = String::new();

    // Read to string
    file.read_to_string(&mut s).unwrap();

    // Turn string to 2d array of tiles
    let mut strct : Grid<Tile> = vec![];
    let mut line : Vec<Tile> = vec![];

    for c in s.chars() {
      if c != '\n' {
        let tile = {
          match c {
            '#' => Tile::new("Wall", ' ', (40, 40, 40), (33, 33, 33), TileType::Wall(WallType::Normal)),
            '.' => Tile::new("Floor", ' ', (27, 27, 27), (20, 20, 20), TileType::Floor(FloorType::Normal)),
            '"' => Tile::new("Tall Grass", '"', (76, 74, 75), (20, 20, 20), TileType::TallGrass),
            _ => panic!("Unknown character: {}", c)
          }
        };
        line.push(tile);
      } else {
        strct.push(line);
        line = vec![];
      }
    }

    // Rotate randomly
    // NOTE: Not done

    // Read details of vector
    let w = strct.len();
    let h = strct[0].len();

    // Add to map if possible
    let x = rng.gen_range(0, w + 1);
    let y = rng.gen_range(0, h + 1);
    
    // Break
    if x + w > self.w - 1 || y + h > self.h - 1 { return; }

    for tx in x..x+w {
      for ty in y..y+h {
        self.grid[tx][ty] = strct[tx-x][ty-y].clone();
      }
    }

  }

  ///
  /// Return a new `Simple`
  ///
  pub fn new(grid: Grid<Tile>) -> Structure {

    // Make a new dungeon with our fresh grid of size `w` by `h`
    let s = Structure {
      grid: grid.clone(),
      w: grid.len(),
      h: grid[0].len(),
    };

    return s;
  }

}

impl Buildable for Structure {

  type Output = Tile;

  fn build(&mut self) -> Grid<Tile> {
    self.add_rand_struct();
    return self.grid.clone();
  }

}
