extern crate rand;
use self::rand::Rng;

// Read files
use std::io::prelude::*;
use std::fs;


use core::renderer::RGB;

use super::Filter;
use core::world::dungeon::map::{self, tile, Tile};

///
/// Structure placer
/// 
/// Generate prefab structures based on files and place them on the grid
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Structure {}

impl Structure {

  ///
  /// Add a random structure
  ///
  fn add_rand_struct(&mut self, grid: &mut map::Grid<Tile>) {

    // RNG
    let mut rng = rand::thread_rng();

    // Create a vector out of collecting the read_dir by mapping the unwrapped paths
    let paths : Vec<_> = fs::read_dir("./strct").unwrap().map(|res| res.unwrap().path()).collect();

    // Choose a random element (aka file from paths)
    let mut file = fs::File::open(rng.choose(&paths).unwrap()).unwrap();

    // Create empty string and read to it
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();

    // Prepare method to store data read from file
    let mut strct : map::Grid<Tile> = vec![];
    let mut line : Vec<Tile> = vec![];

    // Read file as characters
    for ch in string.chars() {
      // If not a newline
      if ch != '\n' {
        // Match tile based on character
        let tile = {
          match ch {
            '#' => Tile::new("Wall", ' ', RGB(40, 40, 40), RGB(33, 33, 33), tile::Type::Wall(tile::Wall::Normal)),
            '.' => Tile::new("Floor", ' ', RGB(27, 27, 27), RGB(20, 20, 20), tile::Type::Floor(tile::Floor::Normal)),
            '"' => Tile::new("Tall Grass", '"', RGB(76, 74, 75), RGB(20, 20, 20), tile::Type::TallGrass),
            _ => panic!("Unknown character: {}", ch)
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

    // Read details of map
    let total_w = grid.len();
    let total_h = grid[0].len();

    // Add to map if possible
    let x = rng.gen_range(0, w + 1);
    let y = rng.gen_range(0, h + 1);
    
    // Break with no change
    if x + w > total_w - 1 || y + h > total_h - 1 { return; }
    
    // Apply change
    for tx in x..x+w {
      for ty in y..y+h {
        grid[tx][ty] = strct[tx-x][ty-y].clone();
      }
    }

  }

  ///
  /// Return a new `Structure`
  ///
  pub fn new() -> Self {
    Structure {}
  }

}

impl Filter for Structure {

  type Output = Tile;

  fn apply(&mut self, grid: &mut map::Grid<Self::Output>) {
    self.add_rand_struct(grid);
  }

}
