extern crate rand;
use self::rand::Rng;

use core::tcod::map::{Map, FovAlgorithm};

use core::object::{Actions, Actor, Creature, Entity};
use core::object::ai::{SimpleAI, TrackerAI, BlinkAI, TalkerAI};
use core::renderer::RGB;

pub mod dungeon;
use self::dungeon::Dungeon;
use self::dungeon::map::{Grid, Tile, TileType, ScentType};

///
/// What value the player sets the scent of nearby tiles to
///
const SC_INC : u8 = 100;

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

///
/// Diameter of scent around creatures, should be odd for best effect
///
const SC_DIAM : isize = 3;
// Upper index for ranges
const SC_DIAM_UPPER : isize = ((SC_DIAM / 2) + 1);
// Lower index for ranges
const SC_DIAM_LOWER : isize = -(SC_DIAM / 2);

// Sound value, needs to be based on type of sound
const SO_INC : u8 = 100;

// Diameter of sound travel
const SO_DIAM : isize = 15;
// Upper index for ranges
const SO_DIAM_UPPER : isize = ((SO_DIAM / 2) + 1);
// Lower index for ranges
const SO_DIAM_LOWER : isize = -(SO_DIAM / 2);

// Water colors
const WATER_COLORS : [(u8, u8, u8); 3] = [
  (51, 133, 255),
  (57, 144, 255),
  (54, 138, 255)
];

pub struct World {
  pub player: Creature,
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
          ScentType::Feline,
          TrackerAI::new()
        )
      )
    );

    creatures.push(
      Box::new(
        Creature::new(
          "blink hound",
          'd',
          {
            let pos = Dungeon::get_valid_location(&g);
            (pos.0 as isize, pos.1 as isize)
          },
          (150, 150, 150), (0, 0, 0),
          ScentType::Canine,
          BlinkAI::new()
        )
      )
    );

    creatures.push(
      Box::new(
        Creature::new(
          "Kurt",
          '@',
          {
            let pos = Dungeon::get_valid_location(&g);
            (pos.0 as isize, pos.1 as isize)
          },
          (200, 200, 200), (0, 0, 0),
          ScentType::Canine,
          TalkerAI::new()
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
          (0, 0, 0),
          TileType::Floor
        );
      }
    }

    let tm = World::new_tcod_map((self.cur_dungeon.width as isize, self.cur_dungeon.height as isize), &self.cur_dungeon);
    self.tcod_map = tm;

    self.creatures = Vec::new();

    self.player.actor.pos.x = (self.cur_dungeon.width / 2) as isize;
    self.player.actor.pos.y = (self.cur_dungeon.height / 2) as isize;
  }

  ///
  /// Return a new player `Entity`
  ///
  #[inline]
  fn fresh_player() -> Creature {
    Creature::new(
      "Player",
      '@',
      (40, 25),
      (255, 255, 255),
      (0, 0, 0),
      ScentType::Player,
      SimpleAI::new()
    )
  }

  ///
  /// Check to see if a specific tile is valid, i.e. walkable and in the map bounds
  ///
  pub fn is_valid(&self, x: isize, y: isize) -> bool {
    let tx = x as usize;
    let ty = y as usize;
    if tx > 0 && tx < self.cur_dungeon.width - 1 && ty > 0 && ty < self.cur_dungeon.height - 1 {
      match self.cur_dungeon.grid[tx][ty].tiletype {
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
    match self.get_tile_at(self.player.actor.pos.x, self.player.actor.pos.y).tiletype {
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
    match self.get_tile_at(self.player.actor.pos.x, self.player.actor.pos.y).tiletype {
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
    self.player.actor.pos.x = start_loc.0 as isize;
    self.player.actor.pos.y = start_loc.1 as isize;

    self.update_fov();
    self.update_water();

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
    w.player.actor.pos.x = start_loc.0 as isize;
    w.player.actor.pos.y = start_loc.1 as isize;
    w.update_fov();

    return w;

  }

  // Get a mutable reference to a tile at a point on the current dungeon
  pub fn get_mut_tile_at(&mut self, x: isize, y: isize) -> &mut Tile {
    &mut self.cur_dungeon.grid[x as usize][y as usize]
  }

  // Get an immutable reference to a tile at a point on the current dungeon
  pub fn get_tile_at(&self, x: isize, y: isize) -> &Tile {
    &self.cur_dungeon.grid[x as usize][y as usize]
  }

  ///
  /// Update the scent map
  ///
  fn update_scent(&mut self) {

    // Create initial bloom around player
    let px = self.player.actor.pos.x;
    let py = self.player.actor.pos.y;
    for nx in SC_DIAM_LOWER..SC_DIAM_UPPER {
      for ny in SC_DIAM_LOWER..SC_DIAM_UPPER {
        if self.is_valid(px - nx, py - ny) {
          for s in &mut self.get_mut_tile_at(px - nx, py - ny).scents {
            match s.scent_type {
              ScentType::Player => {
                s.val = SC_INC
              }
              _ => {}
            }
          }
        }
      }
    }

    // Save information about creatures
    // We can't do a self.get_tile_at due to the fact we iterate over a &self
    // but need a &mut self for that function.
    let mut cinf = vec![];
    for c in &self.creatures {
      let cx = c.actor.pos.x;
      let cy = c.actor.pos.y;
      let st = c.stats.scent_type.clone();
      cinf.push((cx, cy, st));
    }

    // For pair in creature information
    for p in &cinf {
      // Unpack
      let cx = p.0;
      let cy = p.1;
      let st = &p.2;
      for nx in SC_DIAM_LOWER..SC_DIAM_UPPER {
        for ny in SC_DIAM_LOWER..SC_DIAM_UPPER {
          if self.is_valid(cx - nx, cy - ny) {
            for s in &mut self.get_mut_tile_at(cx - nx, cy - ny).scents {
              if &s.scent_type == st {
                s.val = SC_INC
              }
            }
          }
        }
      }
    }

    // Create individual averages for each scent

    // Create buffer for scent updating, only create one
    // because we never change it
    let buffer = self.cur_dungeon.grid.clone();

    for s in 0..ScentType::Num as usize {

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
          if self.is_valid(x as isize, y as isize) {
            self.cur_dungeon.grid[x][y].scents[s].val = avg_of_neighbors(x, y) as u8;
          }
        }
      }
    }

  }

  fn update_sound(&mut self) {

    for x in 0..self.cur_dungeon.width {
      for y in 0..self.cur_dungeon.height {
        self.cur_dungeon.grid[x][y].sound = 0;
      }
    }

    let dist = |obj: &Actor, x: isize, y: isize| -> isize {
      (((obj.pos.x - x).pow(2) + (obj.pos.y - y).pow(2)) as f32).sqrt().floor() as isize
    };

    // Create initial bloom around player
    match &self.player.state {
        &Actions::Move => {
          let px = self.player.actor.pos.x;
          let py = self.player.actor.pos.y;
          for nx in SO_DIAM_LOWER..SO_DIAM_UPPER {
            for ny in SO_DIAM_LOWER..SO_DIAM_UPPER {
              if self.is_valid(px - nx, py - ny) {
                self.get_mut_tile_at(px - nx, py - ny).sound = SO_INC - ((dist(&self.player.actor, px - nx, py - ny)) * (SO_DIAM / 2)) as u8;
              }
            }
          }
        },
        _ => {},
    }

    // Save information about creatures
    // We can't do a self.get_tile_at due to the fact we iterate over a &self
    // but need a &mut self for that function.
    let mut cinf = vec![];
    for c in &self.creatures {
      let cx = c.actor.pos.x;
      let cy = c.actor.pos.y;
      let f = c.actor.clone();
      let s = c.state.clone();
      cinf.push((cx, cy, f, s));
    }

    // For pair in creature information
    for p in &cinf {
      // Unpack
      let s = &p.3;
      match s {
        &Actions::Move => {
          let cx = p.0;
          let cy = p.1;
          let f = &p.2;
          for nx in SO_DIAM_LOWER..SO_DIAM_UPPER {
            for ny in SO_DIAM_LOWER..SO_DIAM_UPPER {
              if self.is_valid(cx - nx, cy - ny) {
                self.get_mut_tile_at(cx - nx, cy - ny).sound = SO_INC - ((dist(&f, cx - nx, cy - ny)) * (SO_DIAM / 2)) as u8;
              }
            }
          }
        },
        _ => {}
      }
    }

  }

  ///
  /// Assign water tiles a new blue color
  /// 
  fn update_water(&mut self) {

    for x in 0..self.cur_dungeon.width {
      for y in 0..self.cur_dungeon.height {
        match self.cur_dungeon.grid[x][y].tiletype {
          TileType::Water => {
            // Water tile should pick a new color from list of colors
            self.cur_dungeon.grid[x][y].set_bg(*rand::thread_rng().choose(&WATER_COLORS).unwrap());
          }
          _ => {}
        }
      }
    }

  }

  pub fn update_fov(&mut self) {
    self.tcod_map.compute_fov(self.player.actor.pos.x as i32, self.player.actor.pos.y as i32, 20, true, FovAlgorithm::Shadow);
  }

  ///
  /// Update the game world
  ///
  pub fn update(&mut self) {
    self.update_fov();
    self.update_scent();
    for c in &mut self.creatures {
      c.take_turn(&self.cur_dungeon.grid, &self.player)
    }
    self.update_water();
    self.update_sound();
  }

}
