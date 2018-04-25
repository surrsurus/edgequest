
pub mod dungeon;
use self::dungeon::Dungeon;
use self::dungeon::map::{Grid, Tile};

use core::object::{Creature, Fighter};
use core::object::ai::SimpleAI;

///
/// What value the player sets the scent of nearby tiles to
/// 
const SC_INC : u8 = 150;

///
/// Affects distance that bloom around player travels
/// 
const SC_BLOOM : f32 = 0.05; 

///
/// Decay value applied to tiles inheriting scent from neighbors
/// 
/// Currently 255/256
/// 
const SC_DECAY : f32 = 0.99609375;

#[derive(Default)]
pub struct World {
  pub player: Fighter,
  pub cur_dungeon: Dungeon,
  pub creatures: Vec<Box<Creature>>,
  pub dungeon_stack: Vec<Dungeon>
}

impl World {

  ///
  /// Create a set of creatures for testing
  ///
  fn create_test_creatures(g: &Grid<Tile>) -> Vec<Box<Creature>> {
    let mut creatures = Vec::<Box<Creature>>::new();
    creatures.push(
      Box::new(
        Creature::new(
          "ant", 
          'a', 
          {
            let pos = Dungeon::get_valid_location(&g);
            (pos.0 as isize, pos.1 as isize)
          }, 
          (255, 0, 0), (0, 0, 0), 
          SimpleAI::new()
        )
      )
    );

    creatures.push(
      Box::new(
        Creature::new(
          "bee", 
          'b', 
          {
            let pos = Dungeon::get_valid_location(&g);
            (pos.0 as isize, pos.1 as isize)
          },  
          (150, 150, 0), (0, 0, 0), 
          SimpleAI::new()
        )
      )
    );

    return creatures;

  }

  ///
  /// Create a basic dungeon for testing
  ///
  fn create_test_dungeon(map_dim: (isize, isize)) -> Dungeon {

    let mut d = Dungeon::new((map_dim.0 as usize, map_dim.1 as usize));

    d.build();

    return d;

  }

  ///
  /// Return a new player `Entity`
  /// 
  #[inline]
  fn fresh_player() -> Fighter {
    Fighter::new(
      "Player",
      '@', 
      (40, 25), 
      (255, 255, 255), 
      (0, 0, 0)
    )
  }

  /// 
  /// Return a new `World`
  /// 
  pub fn new(map_dim: (isize, isize)) -> World {

    let d = World::create_test_dungeon(map_dim);
    let g = d.grid.clone();
    
    let mut w = World {
      player: World::fresh_player(), 
      cur_dungeon: d,
      creatures: World::create_test_creatures(&g),
      dungeon_stack: Vec::new()
    };
      
    let start_loc = Dungeon::get_valid_location(&w.cur_dungeon.grid);
    w.player.pos.x = start_loc.0 as isize;
    w.player.pos.y = start_loc.1 as isize;

    return w;
  
  }

  ///
  /// Update the scent map
  ///
  fn update_scent(&mut self) {

    // Create initial bloom around player
    for nx in -1..2 {
      for ny in -1..2 {
        if self.cur_dungeon.is_valid((self.player.pos.x - nx) as usize, (self.player.pos.y - ny) as usize) {
          self.cur_dungeon.grid[(self.player.pos.x - nx) as usize][(self.player.pos.y - ny) as usize].scent = SC_INC;
        }
      }
    }

    // Make creatures smell
    for c in &self.creatures {
      for nx in -1..2 {
        for ny in -1..2 {
          if self.cur_dungeon.is_valid((c.fighter.pos.x - nx) as usize, (c.fighter.pos.y - ny) as usize) {
            self.cur_dungeon.grid[(c.fighter.pos.x - nx) as usize][(c.fighter.pos.y - ny) as usize].scent = SC_INC;
          }
        }
      }
    }

    // Create buffer
    let buffer = self.cur_dungeon.grid.clone();

    let filter = |tile: &Tile| -> f32 {
      // So, interestingly, if a tile has no scent and is given 0.0 scent after the filter,
      // it creates square scents that travel further, though for some reason a 0.1 value there creates
      // very nice circular scents... I assume this is due to averages now being fuzzy in terms of accuracy?
      if tile.scent == 0 { 0.1 } else { 1.0 }
    };

    // Return an f32 value that is the average value of `Scent`s surrounding the desired position, with a slight decay factor  
    // This is not a "true" average of all neighboring `Scent`s.
    let avg_of_neighbors = |x: usize, y: usize| -> f32 {
      // Add all tile values
      (
      buffer[x - 1][  y  ].scent as f32 +
      buffer[x + 1][  y  ].scent as f32 +
      buffer[  x  ][y - 1].scent as f32 +
      buffer[  x  ][y + 1].scent as f32 +
      buffer[x + 1][y + 1].scent as f32 +
      buffer[x - 1][y - 1].scent as f32 +
      buffer[x + 1][y - 1].scent as f32 +
      buffer[x - 1][y + 1].scent as f32
      ) / 
      
      // Divide by num tiles present, to get the average
      // Add a little bit more on top to make the bloom around player larger
      (((
      filter(&buffer[x - 1][  y  ]) +
      filter(&buffer[x + 1][  y  ]) +
      filter(&buffer[  x  ][y - 1]) +
      filter(&buffer[  x  ][y + 1]) +
      filter(&buffer[x + 1][y + 1]) +
      filter(&buffer[x - 1][y - 1]) +
      filter(&buffer[x + 1][y - 1]) +
      filter(&buffer[x - 1][y + 1]
      )) + SC_BLOOM) 
      
      // Decay factor
      * SC_DECAY)
    };

    // Change values of map based on averages from the buffer
    for x in 0..self.cur_dungeon.width {
      for y in 0..self.cur_dungeon.height {
        if self.cur_dungeon.is_valid(x, y) {
          self.cur_dungeon.grid[x][y].scent = avg_of_neighbors(x, y) as u8;
        }
      }
    }

  }

  ///
  /// Update the game world
  /// 
  pub fn update(&mut self) {
    self.update_scent();
    for c in &mut self.creatures {
      c.take_turn(&self.cur_dungeon.grid, &self.player)
    }
  }
}