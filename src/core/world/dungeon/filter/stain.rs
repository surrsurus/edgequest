extern crate rand;
use self::rand::Rng;

use core::renderer::RGB;

use super::Filter;

use super::map::{Grid, Measurable, tile, Tile};

// Configuration

// Should be divisible by 2
pub const VISCERA_DIAMETER : usize = 5;

const VISCERA_FG : u8 = 40;
const VISCERA_BG : u8 = 50;

const MOSS_FG : u8 = 5;
const MOSS_BG : u8 = 10;

const FUNGUS_FG : u8 = 20;
const FUNGUS_BG : u8 = 40;

const CORRUPTION_FG : u8 = 100;
const CORRUPTION_BG : u8 = 100;

pub enum StainType {
  Viscera,
  Moss,
  Fungus,
  Corruption
}

///
/// Stain 
/// 
/// 'Stains' are a graphical effect that permanently alters the colors of the tiles it is applied to.
/// Multiple variations are possible with customizable radii, chance, and color ammount. 
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Stain;

impl Stain {

  ///
  /// Add stain on a tile 
  ///
  pub fn stain_linear(x: usize, y: usize, d: usize, grid: &mut Grid<Tile>, chance: usize, stain_type: StainType) {

    let mut rng = rand::thread_rng();

    // Coinflip to decide if a tile gets blood
    if rng.gen_range(0, chance) == 0 {
      // Match walls or floors only
      match grid[x - (d/2)][y - (d/2)].tiletype {
        tile::Type::Floor(_) => {

          match stain_type {

            StainType::Viscera => {
              grid[x - (d/2)][y - (d/2)].set_name("Bloody Floor");
              grid[x - (d/2)][y - (d/2)].fg += RGB(12 + rng.gen_range(0, VISCERA_FG), 0, 0);
              grid[x - (d/2)][y - (d/2)].bg += RGB(22 + rng.gen_range(0, VISCERA_BG), 0, 0);
            },
            StainType::Moss => {
              grid[x - (d/2)][y - (d/2)].set_name("Mossy Floor");
              grid[x - (d/2)][y - (d/2)].fg += RGB(0, rng.gen_range(0, MOSS_FG), 0);
              grid[x - (d/2)][y - (d/2)].bg += RGB(0, rng.gen_range(0, MOSS_BG), 0);
            },
            StainType::Fungus => {
              grid[x - (d/2)][y - (d/2)].set_name("Fungal Bloom");
              let fg = rng.gen_range(0, FUNGUS_FG);
              let bg = rng.gen_range(0, FUNGUS_BG);
              grid[x - (d/2)][y - (d/2)].fg += RGB(fg, 0, fg);
              grid[x - (d/2)][y - (d/2)].bg += RGB(fg, 0, bg);
            },
            StainType::Corruption => {
              grid[x - (d/2)][y - (d/2)].set_name("Corrupted Floor");
              let fg = rng.gen_range(0, CORRUPTION_FG);
              let bg = rng.gen_range(0, CORRUPTION_BG);
              grid[x - (d/2)][y - (d/2)].fg -= RGB(fg, fg, fg);
              grid[x - (d/2)][y - (d/2)].bg -= RGB(bg, bg, bg);
            }

          }

        },
        tile::Type::Wall(_) => {

          match stain_type {

            StainType::Viscera => {
              grid[x - (d/2)][y - (d/2)].set_name("Bloody Wall");
              grid[x - (d/2)][y - (d/2)].fg += RGB(rng.gen_range(0, VISCERA_FG), 0, 0);
              grid[x - (d/2)][y - (d/2)].bg += RGB(rng.gen_range(0, VISCERA_BG), 0, 0);
            },
            StainType::Moss => {
              grid[x - (d/2)][y - (d/2)].set_name("Mossy Wall");
              grid[x - (d/2)][y - (d/2)].fg += RGB(0, rng.gen_range(0, MOSS_FG), 0);
              grid[x - (d/2)][y - (d/2)].bg += RGB(0, rng.gen_range(0, MOSS_BG), 0);
            },
            StainType::Fungus => (),
            StainType::Corruption => {
              grid[x - (d/2)][y - (d/2)].set_name("Corrupted Wall");
              let fg = rng.gen_range(0, CORRUPTION_FG);
              let bg = rng.gen_range(0, CORRUPTION_BG);
              grid[x - (d/2)][y - (d/2)].fg -= RGB(fg, fg, fg);
              grid[x - (d/2)][y - (d/2)].bg -= RGB(bg, bg, bg);
            }

          }

        }
        _ => {}
      }
    }

  }

  pub fn add_viscera(x: usize, y: usize, d: usize, grid: &mut Grid<Tile>) {
    for i in 0..d {
      for j in 0..d {
        Stain::stain_linear(x + i, y + j, d, grid, 2, StainType::Viscera);
      }
    }
  }

    pub fn add_moss(x: usize, y: usize, d: usize, grid: &mut Grid<Tile>) {
    for i in 0..d {
      for j in 0..d {
        Stain::stain_linear(x + i, y + j, d, grid, 2, StainType::Moss);
      }
    }
  }

    pub fn add_fungus(x: usize, y: usize, d: usize, grid: &mut Grid<Tile>) {
    for i in 0..d {
      for j in 0..d {
        Stain::stain_linear(x + i, y + j, d, grid, 3, StainType::Fungus);
      }
    }
  }

  pub fn add_corruption(x: usize, y: usize, d: usize, grid: &mut Grid<Tile>) {
    for i in 0..d {
      for j in 0..d {
        Stain::stain_linear(x + i, y + j, d, grid, 1, StainType::Corruption);
      }
    }
  }

  /// 
  /// Return a new `Stain`
  /// 
  pub fn new() -> Self {
    Stain {}
  }

}

impl Filter for Stain {

  type Output = Tile;

  fn apply(&mut self, grid: &mut Grid<Self::Output>) {

    debugln!("stain", "spreading gore randomly...");

    let mut rng = rand::thread_rng();

    // Generate moss  
    for _ in 0..rng.gen_range(1, 3) {

      // Get a random x y
      let x = rng.gen_range(15, grid.width() - 15 - 1);
      let y = rng.gen_range(15, grid.height() - 15 - 1);

      Stain::add_moss(x, y, 15, grid);

    }

    // Generate fungus  
    for _ in 0..rng.gen_range(1, 3) {

      // Get a random x y
      let x = rng.gen_range(7, grid.width() - 7 - 1);
      let y = rng.gen_range(7, grid.height() - 7 - 1);

      Stain::add_fungus(x, y, 7, grid);

    }

    // Generate corruption  
    for _ in 0..1 {

      // Get a random x y
      let x = rng.gen_range(9, grid.width() - 9 - 1);
      let y = rng.gen_range(9, grid.height() - 9 - 1);

      Stain::add_corruption(x, y, 9, grid);

    }

    // Generate blood stains 
    for _ in 0..rng.gen_range(0, 2) {

      // Get a random x y
      let x = rng.gen_range(VISCERA_DIAMETER, grid.width() - VISCERA_DIAMETER - 1);
      let y = rng.gen_range(VISCERA_DIAMETER, grid.height() - VISCERA_DIAMETER - 1);

      Stain::add_viscera(x, y, VISCERA_DIAMETER, grid);

    }

  }

}