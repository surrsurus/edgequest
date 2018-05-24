
pub mod dungeon;
use self::dungeon::Dungeon;
use self::dungeon::map::{Grid, Tile, TileType, ScentType, SCENT_TYPES};

use core::object::{Creature, Fighter, Entity, RGB};
use core::object::ai::{SimpleAI, TrackerAI};

use core::tcod::map::Map;


///
/// What value the player sets the scent of nearby tiles to
/// 
const SC_INC : u8 = 150;

///
/// Affects bloom distance. Higher values means less bloom
/// 
const SC_BLOOM_CUTOFF : f32 = 0.05; 

///
/// Decay value applied to tiles inheriting scent from neighbors
/// 
/// Currently 255/256
/// 
const SC_DECAY : f32 = 0.99609375;

pub struct World {
  pub player: Fighter,
  pub cur_dungeon: Dungeon,
  pub creatures: Vec<Box<Creature>>,
  pub dungeon_stack: Vec<Dungeon>,
  pub tcod_map: Map
  // Need to add http://tomassedovic.github.io/tcod-rs/tcod/map/struct.Map.html
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
          ScentType::Insectoid,
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
          ScentType::Insectoid,
          SimpleAI::new()
        )
      )
    );

    creatures.push(
      Box::new(
        Creature::new(
          "cat", 
          'c', 
          {
            let pos = Dungeon::get_valid_location(&g);
            (pos.0 as isize, pos.1 as isize)
          },  
          (150, 0, 150), (0, 0, 0), 
          ScentType::Mammalian,
          TrackerAI::new()
        )
      )
    );

    creatures.push(
      Box::new(
        Creature::new(
          "dog", 
          'd', 
          {
            let pos = Dungeon::get_valid_location(&g);
            (pos.0 as isize, pos.1 as isize)
          },  
          (150, 150, 150), (0, 0, 0), 
          ScentType::Mammalian,
          TrackerAI::new()
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
  /// Empty out the floor
  ///
  pub fn test_empty(&mut self) {
    
    for x in 0..self.cur_dungeon.width {
      for y in 0..self.cur_dungeon.height {
        self.cur_dungeon.grid[x][y] = Tile::new(
          "Test",
          ' ',
          (0, 0, 0),
          (0, 255, 0),
          TileType::Floor
        );
      }
    }

    let tm = World::new_tcod_map((self.cur_dungeon.width as isize, self.cur_dungeon.height as isize), &self.cur_dungeon);
    self.tcod_map = tm;
  
    self.creatures = Vec::new();

    self.player.pos.x = (self.cur_dungeon.width / 2) as isize;
    self.player.pos.y = (self.cur_dungeon.height / 2) as isize;
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
  /// Check to see if a specific tile is valid, i.e. walkable and in the map bounds
  ///
  pub fn is_valid(&self, x: usize, y: usize) -> bool {
    if x > 0 && x + 1 < self.cur_dungeon.width && y > 0 && y + 1 < self.cur_dungeon.height {
      match self.cur_dungeon.grid[x][y].tiletype {
        TileType::Floor | TileType::DownStair | TileType::UpStair | TileType::Water => return true,
        _ => {}
      }
    }
    return false;
  }

  ///
  /// Return the bg color of a tile at a point
  ///
  /// NOTE: Clearly does not give a fuck if you go oob, probably should change
  ///
  pub fn get_bg_color_at(&self, x: usize, y: usize) -> RGB {

    self.cur_dungeon.grid[x][y].get_bg()

  }

  ///
  /// Go downstairs
  ///
  pub fn can_go_down(&self) -> bool {
    match self.cur_dungeon.grid[self.player.pos.x as usize][self.player.pos.y as usize].tiletype {
      TileType::DownStair => return true,
      _ => return false
    }
  }
  
  pub fn go_down(&mut self) {
    self.test_traverse();
  }

  ///
  /// Go upstairs
  ///
  pub fn can_go_up(&self) -> bool {
    match self.cur_dungeon.grid[self.player.pos.x as usize][self.player.pos.y as usize].tiletype {
      TileType::UpStair => return true,
      _ => return false
    }
  }

  pub fn go_up(&mut self) {
    self.test_traverse();
  }

  ///
  /// Temporary function for stair traversal. In the future floors will need to be saved
  ///
  pub fn test_traverse(&mut self) {
    let d = World::create_test_dungeon((self.cur_dungeon.width as isize, self.cur_dungeon.height as isize));
    let g = d.grid.clone();
    let tm = World::new_tcod_map((self.cur_dungeon.width as isize, self.cur_dungeon.height as isize), &d);
    
    self.cur_dungeon = d;
    self.creatures = World::create_test_creatures(&g);
    self.tcod_map = tm;

    let start_loc = Dungeon::get_valid_location(&self.cur_dungeon.grid);
    self.player.pos.x = start_loc.0 as isize;
    self.player.pos.y = start_loc.1 as isize;
  }

  ///
  /// Return a tcod map based on dungeon features (Essentially what walls you can walk and see through)
  ///
  pub fn new_tcod_map(map_dim: (isize, isize), dungeon: &Dungeon) -> Map {
    let mut tm = Map::new(map_dim.0 as i32, map_dim.1 as i32);

    for x in 0..dungeon.width {
      for y in 0..dungeon.height {
        match dungeon.grid[x][y].tiletype {
          TileType::Wall => {
            tm.set(x as i32, y as i32, false, false);
          },
          _ => {
            tm.set(x as i32, y as i32, true, true);
          }
        }
      }
    }

    return tm;

  }


  /// 
  /// Return a new `World`
  /// 
  pub fn new(map_dim: (isize, isize)) -> World {

    // Create a basic dungeon, tcod map from that dungeon, and a grid we can
    // put test creatures on.
    let d = World::create_test_dungeon(map_dim);
    let g = d.grid.clone();
    let tm =  World::new_tcod_map(map_dim, &d);

    let mut w = World {
      player: World::fresh_player(), 
      cur_dungeon: d,
      creatures: World::create_test_creatures(&g),
      dungeon_stack: Vec::new(),
      tcod_map: tm
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
        if self.is_valid((self.player.pos.x - nx) as usize, (self.player.pos.y - ny) as usize) {
          self.cur_dungeon.grid[(self.player.pos.x - nx) as usize][(self.player.pos.y - ny) as usize].scents[0].val = SC_INC;
        }
      }
    }

    // Make creatures smell based on their smell type
    for c in &self.creatures {
      for nx in -1..2 {
        for ny in -1..2 {
          if self.is_valid((c.fighter.pos.x - nx) as usize, (c.fighter.pos.y - ny) as usize) {
            match c.scent_type {
              ScentType::Insectoid => {
                self.cur_dungeon.grid[(c.fighter.pos.x - nx) as usize][(c.fighter.pos.y - ny) as usize].scents[1].val = SC_INC;
              },
              ScentType::Mammalian => {
                self.cur_dungeon.grid[(c.fighter.pos.x - nx) as usize][(c.fighter.pos.y - ny) as usize].scents[2].val = SC_INC;
              },
              _ => {unreachable!("Should never have a scent type that isn't defined here")}
            }
          }
        }
      }
    }

    for s in 0..SCENT_TYPES {
      // Create buffer
      let buffer = self.cur_dungeon.grid.clone();

      let filter = |tile: &Tile| -> f32 {
        // So, interestingly, if a tile has no scent and is given 0.0 scent after the filter,
        // it creates square scents that travel further, though for some reason a 0.1 value there creates
        // very nice circular scents... I assume this is due to averages now being fuzzy in terms of accuracy?
        if tile.scents[s].val == 0 { 0.1 } else { 1.0 }
      };

      // Return an f32 value that is the average value of `Scent`s surrounding the desired position, with a slight decay factor  
      // This is not a "true" average of all neighboring `Scent`s.
      let avg_of_neighbors = |x: usize, y: usize| -> f32 {
        // Add all tile values
        (
        buffer[x - 1][  y  ].scents[s].val as f32 +
        buffer[x + 1][  y  ].scents[s].val as f32 +
        buffer[  x  ][y - 1].scents[s].val as f32 +
        buffer[  x  ][y + 1].scents[s].val as f32 +
        buffer[x + 1][y + 1].scents[s].val as f32 +
        buffer[x - 1][y - 1].scents[s].val as f32 +
        buffer[x + 1][y - 1].scents[s].val as f32 +
        buffer[x - 1][y + 1].scents[s].val as f32
        ) / 
        
        // Divide by num tiles present, to get the average
        // Add some value to reduce size of bloom 
        (((
        filter(&buffer[x - 1][  y  ]) +
        filter(&buffer[x + 1][  y  ]) +
        filter(&buffer[  x  ][y - 1]) +
        filter(&buffer[  x  ][y + 1]) +
        filter(&buffer[x + 1][y + 1]) +
        filter(&buffer[x - 1][y - 1]) +
        filter(&buffer[x + 1][y - 1]) +
        filter(&buffer[x - 1][y + 1]
        )) + SC_BLOOM_CUTOFF) 
        
        // Decay factor
        * SC_DECAY)
      };

      // Change values of map based on averages from the buffer
      for x in 0..self.cur_dungeon.width {
        for y in 0..self.cur_dungeon.height {
          if self.is_valid(x, y) {
            self.cur_dungeon.grid[x][y].scents[s].val = avg_of_neighbors(x, y) as u8;
          }
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