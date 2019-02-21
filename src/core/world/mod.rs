// extern crate serde_json;
// JSON Parsing and Construction
// https://github.com/serde-rs/json
// use self::serde_json::Value;
// use self::serde_json::map;

// Use to read files
// use std::fs::File;
// use std::io::prelude::*;

extern crate rand;
use self::rand::Rng;

use core::tcod::map::{Map, FovAlgorithm};


use core::creature::{ai, Actions, Creature, Stats};

use core::item::{Item, ItemProperty, Money};

use core::renderer::{Renderable, RGB};

use core::log;

pub mod dungeon;
use self::dungeon::{Dungeon, map::{self, Pos, tile, Tile}};

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

///
/// Represent a floor in the dungeon
///
#[derive(Default, Clone)]
pub struct Floor {
  pub dun: Dungeon,
  // Creatures need to be boxed because they hold a trait object, which has an undefined size.
  // Whenever you create a creature, just slap it into Box::new() and it works
  pub creatures: Vec<Box<Creature>>,
  // Items on the floor
  pub items: Vec<Item>
}

impl Floor {
  pub fn new(dun: Dungeon, creatures: Vec<Box<Creature>>) -> Self {
    Floor {
      dun: dun,
      creatures: creatures,
      items: vec![]
    }
  }

}

pub struct World {
  pub player: Creature,
  pub floor: Floor,
  pub floor_stack: Vec<Floor>,
  pub floor_num: usize,
  // http://tomassedovic.github.io/tcod-rs/tcod/map/struct.Map.html
  pub tcod_map: Map
}

impl World {

  ///
  /// Create a set of creatures for testing. 100% temporary
  ///
  fn create_test_creatures(g: &map::Grid<Tile>) -> Vec<Box<Creature>> {
    
    let mut creatures = Vec::<Box<Creature>>::new();

    creatures.push(
      Box::new(
        Creature::new(
          "ant",
          'a',
          Dungeon::get_valid_location(g),
          RGB(150, 0, 0), RGB(0, 0, 0),
          Stats::new(
            0,
            0,
            0,
            0,
            tile::Scent::Insectoid
          ),
          ai::SimpleAI::new()
        )
      )
    );

    creatures.push(
      Box::new(
        Creature::new(
          "bee",
          'b',
          Dungeon::get_valid_location(g),
          RGB(150, 150, 0), RGB(0, 0, 0),
          Stats::new(
            0,
            0,
            0,
            0,
            tile::Scent::Insectoid
          ),
          ai::SimpleAI::new()
        )
      )
    );

    creatures.push(
      Box::new(
        Creature::new(
          "cat",
          'c',
          Dungeon::get_valid_location(g),
          RGB(150, 0, 150), RGB(0, 0, 0),
          Stats::new(
            0,
            0,
            0,
            5,
            tile::Scent::Feline
          ),
          ai::TrackerAI::new()
        )
      )
    );

    creatures.push(
      Box::new(
        Creature::new(
          "blink hound",
          'd',
          Dungeon::get_valid_location(g),
          RGB(150, 150, 150), RGB(0, 0, 0),
          Stats::new(
            0,
            0,
            0,
            20,
            tile::Scent::Canine
          ),
          ai::BlinkAI::new()
        )
      )
    );

    creatures.push(
      Box::new(
        Creature::new(
          "Kurt",
          '@',
          Dungeon::get_valid_location(g),
          RGB(200, 200, 200), RGB(0, 0, 0),
          Stats::new(
            0,
            0,
            0,
            50,
            tile::Scent::Canine
          ),
          ai::TalkerAI::new()
        )
      )
    );

    creatures.push(
      Box::new(
        Creature::new(
          "Echidna",
          'e',
          Dungeon::get_valid_location(g),
          RGB(50, 50, 200), RGB(0, 0, 0),
          Stats::new(
            0,
            0,
            0,
            15,
            tile::Scent::Canine
          ),
          ai::SmellerAI::new()
        )
      )
    );

    return creatures;

  }

  ///
  /// Create a basic dungeon for testing
  ///
  fn create_test_dungeon(map_dim: Pos) -> Dungeon {
    return Dungeon::new(map_dim).build();
  }

  ///
  /// Return a new player `Creature`
  ///
  #[inline]
  fn new_player() -> Creature {
    Creature::new(
      "Player",
      '@',
      Pos::new(40, 25),
      RGB(255, 255, 255), RGB(0, 0, 0),
      Stats::new(
        0,
        0,
        0,
        20,
        tile::Scent::Player
      ),
      ai::PlayerAI::new()
    )
  }


  ///
  /// Empty out the floor
  ///
  pub fn test_empty(&mut self) {

    for x in 0..self.floor.dun.width {
      for y in 0..self.floor.dun.height {
        self.floor.dun[x][y] = tile::generic_floor();
      }
    }

    let tcod_map = World::new_tcod_map(self.floor.dun.get_bounds_pos(), &self.floor.dun);
    self.tcod_map = tcod_map;

    self.floor.creatures = Vec::new();
    self.floor.items =     Vec::new();

    self.player.actor.pos.x = (self.floor.dun.width / 2) as isize;
    self.player.actor.pos.y = (self.floor.dun.height / 2) as isize;
    
  }

  ///
  /// Check to see if a specific position is valid, i.e. tile::walkable and in the map bounds
  ///
  pub fn is_valid_pos(&self, x: isize, y: isize) -> bool {
    
    // Conversion to usize
    let ux = x as usize;
    let uy = y as usize;

    return ux > 0 && ux < self.floor.dun.width - 1 && uy > 0 && uy < self.floor.dun.height - 1 && tile::walkable(&self.floor.dun[ux][uy]);

  }

  ///
  /// Check for dead creatures
  /// 
  pub fn check_death(&mut self) {
    self.floor.creatures.retain( |creature| creature.state != Actions::Die )
  }

  ///
  /// See if player stepped on items
  ///
  pub fn check_items(&mut self) {

    // Don't repeat if we already know what's under foot
    match self.player.state {
      Actions::Wait | Actions::Die | Actions::Unknown | Actions::UpStair | Actions::DownStair => return,
      _ => ()
    }

    // Get all items on the same tile as the player
    let items_at_feet = self.floor.items.iter().filter(|item| item.pos == self.player.actor.pos);

    // Possible stuff for stacking items

    // // Proceed to stack like items
    // let mut stacked_items = HashMap::new();

    // for item in items_at_feet {
    //   if !stacked_items.contains_key(item.get_id()) {
    //     stacked_items.insert(item.get_id(), 1);
    //   } else {
    //     *stacked_items.get_mut(item.get_id()).unwrap() += 1;
    //   }
    // }

    // for (id, value) in &stacked_items {
    //   log!( (format!() , RGB(200, 200, 130) ) );
    // }

    for item in items_at_feet {
      if item.quantity > 1 {
        log!( (Box::leak(format!("You see {} {}s here", item.quantity, item.get_id()).into_boxed_str()), item.get_fg()) );
      } else {
        log!( (Box::leak(format!("You see a {} here", item.get_id()).into_boxed_str()), item.get_fg()) );
      }
    }

  } 

  ///
  /// Check to see if a tile is a trap
  /// 
  /// Should only be called after checking tile validity to avoid OOB errors
  /// 
  pub fn check_traps(&mut self) {
    
    match &self.floor.dun[self.player.actor.pos].tiletype.clone() {

      // We only care about traps, and this matches every trap
      tile::Type::Trap(trap) => {
        
        log!(("You step on a trap!", RGB(255, 0, 0)));

        // Match the type of trap
        match trap {

          // Memory loss causes all tiles to become unseen, effectively losing all mapping progress
          tile::Trap::MemoryLoss => {

            log!(("You lose your memory", RGB(255, 255, 0)));
            
            for tile in self.floor.dun.grid.iter_mut().flatten() {
              tile.seen = false;
            }

          },

          // Fall down a floor or three
          tile::Trap::Shaft => {

            log!(("You fall down a shaft!", RGB(200, 50, 20)));
            
            for _floors in 0..rand::thread_rng().gen_range(1, 4) {
              self.go_down();
            }

          },

          // Turn creature a new color
          tile::Trap::PaintBomb => {

            let mut rng = rand::thread_rng();

            let col = RGB(rng.gen_range(1, 255), rng.gen_range(1, 255), rng.gen_range(1, 255));

            log!(("It's a paint bomb!", RGB(100, 100, 100)));

            log!(("You look different!", col));

            self.player.actor.set_fg(col);

          }

          // Move randomly on map
          tile::Trap::Teleport => {

            log!(("It's a teleporter!", RGB(50, 127, 200)));

            self.player.actor.pos = Dungeon::get_valid_location(&self.floor.dun.grid);

          }

        }

      },
      _ => {}
    }

    // Did a creature step on a trap
    for creature in &mut self.floor.creatures {
      match &self.floor.dun[creature.actor.pos].tiletype.clone() {
        // We only care about traps, and this matches every trap
        tile::Type::Trap(trap) => {
          // Match the type of trap
          match trap {

            // Not sure how this affects monsters
            tile::Trap::MemoryLoss => {},

            // Fall down and die I guess
            tile::Trap::Shaft => {

              log!(("You hear a trap door open!", RGB(200, 50, 20)));
              
              // Not sure what to do with the creature here...
              creature.state = Actions::Die;

            },

            // Turn creature a new color
            tile::Trap::PaintBomb => {

              let mut rng = rand::thread_rng();

              let col = RGB(rng.gen_range(1, 255), rng.gen_range(1, 255), rng.gen_range(1, 255));

              log!(("You hear an explosion!", RGB(100, 100, 100)));

              creature.actor.set_fg(col);

            }

            // Move randomly on map
            tile::Trap::Teleport => {

              log!(("You hear the hum of a teleporter!", RGB(50, 127, 200)));

              creature.actor.pos = Dungeon::get_valid_location(&self.floor.dun.grid);

            }
          }
        }
        _ => ()
      }
    }

  }

  ///
  /// Return the bg color of a tile at a point
  ///
  /// NOTE: Clearly does not give a fuck if you go OOB, probably should change
  /// Interestingly, it's only used by the renderer and that can't display stuff out of bounds (thanks, camera)
  /// so maybe it's not important to have needless code?
  ///
  pub fn get_bg_color_at(&self, pos: Pos) -> RGB {
    self.floor.dun[pos].get_bg()
  }

  ///
  /// Go downstairs if possible
  ///
  pub fn go_down(&mut self) {

    if self.floor_num <= self.floor_stack.len() {
      self.floor_stack[self.floor_num] = self.floor.clone();
    }
    self.floor_num += 1;
    self.test_traverse();

  }

  ///
  /// Save the current floor and go up one floor
  ///
  pub fn go_up(&mut self) {
    
    // Be sure we aren't going to mess something up
    assert!(self.floor_num != 0);

    self.floor_stack[self.floor_num] = self.floor.clone();
    self.floor_num -= 1;
    self.test_traverse();

  }
  
  ///
  /// See if the player is able to go up on the current tile and draw some stuff to the log
  /// 
  pub fn player_go_up(&mut self) {

    match self.get_tile_at(self.player.actor.pos.x, self.player.actor.pos.y).tiletype {
      tile::Type::Stair(tile::Stair::UpStair(_)) => {
        if self.floor_num != 0 {
          self.go_up();
          log!(("You bravely venture forth...", RGB(255, 255, 200)));
        } else {
          log!(("You are not allowed to turn back now...", RGB(100, 50, 25)));
        }
      },
      _ => log!(("You can't go up here", RGB(150, 150, 150)))
    }

  }

  ///
  /// See if the player is able to go down on the current tile and draw some stuff to the log
  /// 
  pub fn player_go_down(&mut self) {
    match self.get_tile_at(self.player.actor.pos.x, self.player.actor.pos.y).tiletype {
      tile::Type::Stair(tile::Stair::DownStair(_)) => {
        self.go_down();
        log!(("You bravely venture forth...", RGB(255, 255, 200)));
      },
      _ => log!(("You can't go down here", RGB(150, 150, 150)))
    }
  }

  ///
  /// Temporary function for stair traversal
  ///
  pub fn test_traverse(&mut self) {
    
    // Create floor var
    let mut floor;

    // If the floor number that we are on is not a floor in the stack,
    // we need to add a new floor to the stack
    if self.floor_num > self.floor_stack.len() - 1 {
      let dun = World::create_test_dungeon(self.floor.dun.get_bounds_pos());
      let grid = dun.grid.clone();
      let creatures = World::create_test_creatures(&grid);
      floor = Floor::new(dun, creatures);
      // Create n gold coins at a valid location
      let gold_loc = Dungeon::get_valid_location(&floor.dun.grid);
      floor.items.push(
        Item::new("gold piece", '$', gold_loc, RGB(238, 232, 170), RGB(0, 0, 0), rand::thread_rng().gen_range(10, 40), ItemProperty::Money(Money::Gold))
      );
      self.floor_stack.push(floor.clone());
    // Otherwise the floor already exists in the stack and can be brought out
    } else {
      floor = self.floor_stack[self.floor_num].clone();
    }

    self.floor = floor;

    self.tcod_map = World::new_tcod_map(self.floor.dun.get_bounds_pos(), &self.floor.dun);

    let start_loc = Dungeon::get_valid_location(&self.floor.dun.grid);
    self.player.actor.pos.x = start_loc.x;
    self.player.actor.pos.y = start_loc.y;

    self.update_fov();

  }

  ///
  /// Return a tcod map based on dungeon features (Essentially what walls you can walk and see through)
  ///
  pub fn new_tcod_map(map_dim: Pos, dungeon: &Dungeon) -> Map {
    let mut tcod_map = Map::new(map_dim.x as i32, map_dim.y as i32);

    // Fill the map in based on what blocks are tile::opaque
    for x in 0..dungeon.width {
      for y in 0..dungeon.height {
        if tile::opaque(&dungeon[x][y]) {
          tcod_map.set(x as i32, y as i32, false, false);
        } else {
          tcod_map.set(x as i32, y as i32, true, true);
        }
      }
    }

    return tcod_map;

  }

  ///
  /// Return a new `World`
  ///
  pub fn new(map_dim: Pos) -> Self {

    // Create a basic dungeon, tcod map from that dungeon, and a grid we can
    // put test creatures on.
    let dun = World::create_test_dungeon(map_dim);
    let grid = dun.grid.clone();
    let tcod_map =  World::new_tcod_map(map_dim, &dun);

    let mut floor = Floor::new(dun, World::create_test_creatures(&grid));

    // Create n gold coins at a valid location
    let gold_loc = Dungeon::get_valid_location(&floor.dun.grid);
    floor.items.push(
      Item::new("gold piece", '$', gold_loc, RGB(238, 232, 170), RGB(0, 0, 0), rand::thread_rng().gen_range(10, 40), ItemProperty::Money(Money::Gold))
    );

    let mut floor_stack = Vec::new();
    floor_stack.push(floor.clone());

    let mut world = World {
      player: World::new_player(),
      floor: floor,
      floor_stack: floor_stack,
      floor_num: 0,
      tcod_map: tcod_map
    };

    world.player.actor.pos = Dungeon::get_valid_location(&world.floor.dun.grid);
    world.update_fov();

    return world;

  }

  // Get a mutable reference to a tile at a point on the current dungeon
  pub fn get_mut_tile_at(&mut self, x: isize, y: isize) -> &mut Tile {
    &mut self.floor.dun[x as usize][y as usize]
  }

  // Get an immutable reference to a tile at a point on the current dungeon
  pub fn get_tile_at(&self, x: isize, y: isize) -> &Tile {
    &self.floor.dun[x as usize][y as usize]
  }

  ///
  /// Find sounds created by creatures
  /// 
  fn find_creature_sounds(&mut self) -> Vec<(Pos, usize)> {

    let mut sounds : Vec<(Pos, usize)> = vec![];

    // Determine if the player made sound by moving
    match &self.player.state {
      Actions::Move => sounds.push((self.player.actor.pos, self.player.stats.weight)),
      Actions::Talk => sounds.push((self.player.actor.pos, 25)),
      _ => {}
    }

    // Determine if any creatures made sound by moving
    for creature in &self.floor.creatures {
      match &creature.state {
        Actions::Move => sounds.push((creature.actor.pos, creature.stats.weight)),
        Actions::Talk => sounds.push((creature.actor.pos, 25)),
        _ => {}
      }
    }

    return sounds;
    
  }

  ///
  /// Update the scent map
  ///
  fn update_scent(&mut self) {

    // Create initial bloom around player
    let player_x = self.player.actor.pos.x;
    let player_y = self.player.actor.pos.y;

    for x in SC_DIAM_LOWER..SC_DIAM_UPPER {
      for y in SC_DIAM_LOWER..SC_DIAM_UPPER {
        if self.is_valid_pos(player_x - x, player_y - y) {
          for scent in &mut self.get_mut_tile_at(player_x - x, player_y - y).scents {
            match scent.scent_type {
              tile::Scent::Player => {
                scent.val = SC_INC
              }
              _ => {}
            }
          }
        }
      }
    }

    // Save information about creatures
    let mut creature_information = vec![];
    for creature in &self.floor.creatures {
      let creature_x = creature.actor.pos.x;
      let creature_y = creature.actor.pos.y;
      let scent_type = creature.stats.scent_type.clone();
      creature_information.push((creature_x, creature_y, scent_type));
    }

    // For tuple in creature information
    for tuple in &creature_information {
      // Unpack
      let creature_x = tuple.0;
      let creature_y = tuple.1;
      let scent_type = &tuple.2;

      for x in SC_DIAM_LOWER..SC_DIAM_UPPER {
        for y in SC_DIAM_LOWER..SC_DIAM_UPPER {
          if self.is_valid_pos(creature_x - x, creature_y - y) {
            for scent in &mut self.get_mut_tile_at(creature_x - x, creature_y - y).scents {
              if &scent.scent_type == scent_type {
                scent.val = SC_INC
              }
            }
          }
        }
      }
    }

    // Create individual averages for each scent

    // Create buffer for scent updating, only create one
    // because we never change it
    let buffer = self.floor.dun.grid.clone();

    for scent_type_idx in 0..tile::Scent::Num as usize {

      let filter = |tile: &Tile| -> f32 {
        // So, interestingly, if a tile has no scent and is given 0.0 scent after the filter,
        // it creates square scents that travel further, though for some reason a 0.1 value there creates
        // very nice circular scents... I assume this is due to averages now being fuzzy in terms of accuracy?
        if tile.scents[scent_type_idx].val == 0 { 0.1 } else { 1.0 }
      };

      // Return an f32 value that is the average value of `Scent`s surrounding the desired position, with a slight decay factor
      // This is not a "true" average of all neighboring `Scent`s.
      let avg_of_neighbors = |x: usize, y: usize| -> f32 {

        // Add all tile values
        (
          buffer[x - 1][  y  ].scents[scent_type_idx].val as f32 +
          buffer[x + 1][  y  ].scents[scent_type_idx].val as f32 +
          buffer[  x  ][y - 1].scents[scent_type_idx].val as f32 +
          buffer[  x  ][y + 1].scents[scent_type_idx].val as f32 +
          buffer[x + 1][y + 1].scents[scent_type_idx].val as f32 +
          buffer[x - 1][y - 1].scents[scent_type_idx].val as f32 +
          buffer[x + 1][y - 1].scents[scent_type_idx].val as f32 +
          buffer[x - 1][y + 1].scents[scent_type_idx].val as f32
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
      for x in 0..self.floor.dun.width {
        for y in 0..self.floor.dun.height {
          if self.is_valid_pos(x as isize, y as isize) {
            self.floor.dun[x][y].scents[scent_type_idx].val = avg_of_neighbors(x, y) as u8;
          }
        }
      }
    }
  }

  ///
  /// Update the sound map
  /// 
  pub fn update_sound(&mut self) {

    // Distance formula for sounds
    let dist = |pos: Pos, x: isize, y: isize| -> usize {
      (((pos.x - x).pow(2) + (pos.y - y).pow(2)) as f32).sqrt().floor() as usize
    };

    // Analyze events to determine sounds
    let sounds : Vec<(Pos, usize)> = self.find_creature_sounds();

    // Other sound generators go here

    // Reset sound to 0
    for x in 0..self.floor.dun.width {
        for y in 0..self.floor.dun.height {
          let mut tile = self.get_mut_tile_at(x as isize, y as isize);
          tile.sound = 0;
        }
    }

    // Expand each sound point-source
    // Sound decrases in intensity proportional
    // to the inverse of distance squared
    for sound in sounds {
      for x in 0..self.floor.dun.width {
        for y in 0..self.floor.dun.height {
          let mut tile = self.get_mut_tile_at(x as isize, y as isize);
          if true {
            tile.sound = tile.sound + (sound.1 / ((dist(sound.0, x as isize, y as isize) + 1).pow(2)));
          }
        }
      }
    }
  }

  ///
  /// Update the fov map from the player's perspective
  /// 
  pub fn update_fov(&mut self) {
    self.tcod_map.compute_fov(self.player.actor.pos.x as i32, self.player.actor.pos.y as i32, 20, true, FovAlgorithm::Shadow);
  }

  ///
  /// Update the game world
  ///
  pub fn update(&mut self) {
    self.update_fov();
    self.update_scent();
    for creature in &mut self.floor.creatures {
      creature.take_turn(&self.floor.dun.grid, &self.player)
    }
    self.check_traps();
    self.check_items();
    self.update_sound();
    self.check_death();
    // self.debug_show_mem();
  }

}
