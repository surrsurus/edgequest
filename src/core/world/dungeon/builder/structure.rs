extern crate rand;
use self::rand::Rng;

// Read files
use std::io::prelude::*;
use std::fs;

use core::world::dungeon::builder::Buildable;

use core::world::dungeon::map::{self, tile, Tile};

use core::renderer::RGB;

///
/// Structure placer
/// 
/// Generate prefab structures based on files and place them on the grid
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Structure {
  grid: map::Grid<Tile>,
  w: usize,
  h: usize,
}

impl Structure {

  ///
  /// Add a random structure
  ///
  fn add_rand_struct(&mut self) {

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

    // Prepare method to store data read from file
    let mut strct : map::Grid<Tile> = vec![];
    let mut line : Vec<Tile> = vec![];

    // Read file as characters
    for c in s.chars() {
      // If not a newline
      if c != '\n' {
        // Match tile based on character
        let tile = {
          match c {
            '#' => Tile::new("Wall", ' ', RGB(40, 40, 40), RGB(33, 33, 33), tile::Type::Wall(tile::Wall::Normal)),
            '.' => Tile::new("Floor", ' ', RGB(27, 27, 27), RGB(20, 20, 20), tile::Type::Floor(tile::Floor::Normal)),
            '"' => Tile::new("Tall Grass", '"', RGB(76, 74, 75), RGB(20, 20, 20), tile::Type::TallGrass),
            _ => panic!("Unknown character: {}", c)
          }
        };
        // Push character to the line
        line.push(tile);
      // If we hit a new line we need to push the line to the tile struct, and empty the line
      } else {
        strct.push(line);
        line = vec![];
      }
    }

    // Rotate randomly

    // (x, y) rotated 90 degrees around (0, 0) is (-y, x).
    // However, vectors are sized in a way that won't allow for negative indexing.
    // Our formula for point tranformation should be:
    // (-y + total x length, x)

    let rot90 = | grid: map::Grid<Tile> | -> map::Grid<Tile> {

      // We could clone but I feel like this way is faster
      let mut rot_grid = map::Grid::<Tile>::new();

      // Measure x on y axis
      for x in 0..grid[0].len() {

        // Fill new vecs with init
        let mut vec = Vec::<Tile>::new();

        // Measure y on x axis
        for y in 0..grid.len() {
          vec.push(
            
            // Rotation performed by following above function
            grid[grid.len() - 1 - y][x].clone()

          );
        }

        rot_grid.push(vec);

      }

      return rot_grid;

    };

    // Perform 1 - 4 rotations
    // NOTE: 4 rotations = starting positon. Might be a good idea to improve this
    for _ in 0..rng.gen_range(0, 4) {
      strct = rot90(strct);
    }

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
  /// Return a new `Structure`
  ///
  pub fn new(grid: map::Grid<Tile>) -> Structure {

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

  fn build(&mut self) -> map::Grid<Tile> {
    self.add_rand_struct();
    return self.grid.clone();
  }

}
