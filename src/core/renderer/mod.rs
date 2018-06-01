//!
//! Metapackage for renderer
//!

use std::char;

// `Console` is needed as Console is a trait that console::Root extends
use core::tcod::{Console, console};

use core::GlobalLog;

use core::world::World;

use core::world::dungeon::Dungeon;
// Used to expliclty reference constants
use core::world::dungeon::map::tile;
use core::world::dungeon::map::{Tile, TileType, ScentType};

use core::object::{Pos, Entity};

// Use camera privately
mod camera;
use self::camera::Camera;

pub mod rgb;
pub use self::rgb::RGB;

///
/// Helper for rendering things to the screen
///
/// Tracks the player and automatically scrolls the screen around to match where they go.
/// This will never try to draw things outside of the given dimensions due to the way it handles
/// determining whether something should be drawn or not.
///
pub struct Renderer {
  // Camera object
  camera: Camera,
  screen: Pos,
  console_height: isize,
  panel_width: isize,
  pub sc_debug: bool,
  pub fov: bool,
  pub so_debug: bool
}

impl Renderer {

  ///
  /// Render for each monster as a visible colored entity
  ///
  pub fn debug_render_scent_map(&mut self, con: &mut console::Root, dungeon: &Dungeon) {

    for x in 0..dungeon.width {
      for y in 0..dungeon.height {
        // Pretty much just random, Player is red, bugs are green, cats are yellow and dogs are blue
        let mut color : (u8, u8, u8) = (
          dungeon.grid[x][y].scents[0].val + 50 + dungeon.grid[x][y].scents[3].val, 
          dungeon.grid[x][y].scents[1].val + 25 + dungeon.grid[x][y].scents[3].val, 
          dungeon.grid[x][y].scents[2].val + 50 
        );
        // Iterate over scents, context of what scent it is isn't necessary
        for s in 0..ScentType::Num as usize {
          if dungeon.grid[x][y].scents[s].val > 0 {
            self.draw_entity(con, Pos::new(x as isize, y as isize), &Tile::new(
              "Debug Scent",
              ' ',
              (255, 255, 255),
              color,
              TileType::Debug
            ));
            break;
          }
        }
      }
    }

  }

  ///
  /// Render sound as a transparent blue entity
  ///
  pub fn debug_render_sound_map(&mut self, con: &mut console::Root, dungeon: &Dungeon) {

    for x in 0..dungeon.width {
      for y in 0..dungeon.height {
        // Color is weighted towards blue
        let mut color : (u8, u8, u8) = (
          dungeon.grid[x][y].get_bg().0, 
          dungeon.grid[x][y].get_bg().1, 
          dungeon.grid[x][y].sound 
        );
        if dungeon.grid[x][y].sound > 0 {
          self.draw_entity(con, Pos::new(x as isize, y as isize), &Tile::new(
            "Debug Sound",
            ' ',
            (255, 255, 255),
            color,
            TileType::Debug
          ));
        }
      }
    }

  }

  ///
  /// Draw all.
  ///
  pub fn draw_all(&mut self, con: &mut console::Root, world: &mut World) {
    
    //
    // Console prep
    //

    // Clear console
    con.clear();

    // Move camera to player's position
    self.camera.move_to(world.player.actor.pos);

    //
    // Draw tiles
    //

    // Draw seen tiles
    for x in 0..world.cur_dungeon.width {
      for y in 0..world.cur_dungeon.height {
        // If fov is on...
        if self.fov {
          // And it's in the FoV
          if world.tcod_map.is_in_fov(x as i32, y as i32) && self.fov {
            match world.cur_dungeon.grid[x][y].tiletype {
              _ => {
                // Draw a tile slightly more vibrant than it actually is
                self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y].yellowish());
              }
            }

            // Mark tile as seen if it's in the FoV
            world.cur_dungeon.grid[x][y].seen = true;

          }

          // And the tile has been seen...
          else if world.cur_dungeon.grid[x][y].seen {
            // Draw certain tiles depending on their types
            match world.cur_dungeon.grid[x][y].tiletype {
              _ => {
                // Draw a tile slightly darker than it actually is
                self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y].darken());
              }
            }
          }

        }

        // [Debug] Otherwise just draw all tiles normally
        else {
         match world.cur_dungeon.grid[x][y].tiletype {
            _ => {
              self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y]);
            }
          }
        }
        
      }
    }

    //
    // Debug options
    //

    // Debug scent
    if self.sc_debug {
      self.debug_render_scent_map(con, &world.cur_dungeon);
    }

    // Debug sound
    if self.so_debug {
      self.debug_render_sound_map(con, &world.cur_dungeon);
    }

    //
    // Draw creatures
    //

    for c in &world.creatures {
      // If fov is on...
      if self.fov {
        // And its in the fov...
        if world.tcod_map.is_in_fov(c.actor.pos.x as i32, c.actor.pos.y as i32) && self.fov {
          self.draw_creature(con, c.actor.pos, &c.actor, world);
        }
      } else {
        self.draw_creature(con, c.actor.pos, &c.actor, world);
      }
    }

    // Draw player. Player is always in the camera since
    // we move the camera over it.
    self.draw_creature(con, world.player.actor.pos, &world.player.actor, world);

    //
    // Draw log
    //

    self.draw_log(con);

    //
    // Draw UI
    //

    for x in 0..self.screen.x {
      con.put_char_ex(
        x as i32,
        (self.screen.y - self.console_height - 1) as i32,
        '-',
        RGB(255, 255, 255).to_tcod(),
        RGB(0, 0, 0).to_tcod()
      );
    }

    for y in 0..(self.screen.y - self.console_height - 1) {
      con.put_char_ex(
        (self.screen.x - self.panel_width - 1) as i32,
        y as i32,
        '|',
        RGB(255, 255, 255).to_tcod(),
        RGB(0, 0, 0).to_tcod()
      );
    }

    con.put_char_ex(
      (self.screen.x - self.panel_width - 1) as i32,
      (self.screen.y - self.console_height - 1) as i32,
      char::from_u32(193).unwrap(),
      RGB(255, 255, 255).to_tcod(),
      RGB(0, 0, 0).to_tcod()
    );

    con.set_default_foreground(RGB(255, 255, 255).to_tcod());

    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      0,
      "Edgequest"
    );

    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      1,
      "This is where we live"
    );

    //
    // Flush changes to root
    //

    con.flush();

  }

  ///
  /// Draw creatures with "transparent" backgrounds
  ///
  pub fn draw_creature(&self, con: &mut console::Root, pos: Pos, ce: &Entity, world: &World) {
    // Check if it's in the camera first
    if !self.camera.is_in_camera(pos) { return }

    // New pos with respect to camera
    let npos = pos + self.camera.pos;
  
    con.put_char_ex(
      npos.x as i32,
      npos.y as i32,
      ce.get_glyph(),
      ce.get_fg().to_tcod(),
      // Backgrounds are just inherited from the world.
      if self.fov {
        (world.get_bg_color_at(pos.x as usize, pos.y as usize) + tile::YELLOW_FAC).to_tcod()
      } else {
        (world.get_bg_color_at(pos.x as usize, pos.y as usize)).to_tcod()
      }
    );

  }

  ///
  /// Put an `Entity` on the console
  ///
  /// * `con` - Tcod `Console`
  /// * `entity` - `Entity`
  ///
  pub fn draw_entity(&self, con: &mut console::Root, pos: Pos, ce: &Entity) {

    // Check if it's in the camera first
    if !self.camera.is_in_camera(pos) { return }

    // New pos with respect to camera
    let pos = pos + self.camera.pos;

    if ce.get_glyph() == ' ' {
      con.set_char_background(
        pos.x as i32,
        pos.y as i32,
        ce.get_bg().to_tcod(),
        console::BackgroundFlag::Set
      );
    } else {
      con.put_char_ex(
        pos.x as i32,
        pos.y as i32,
        ce.get_glyph(),
        ce.get_fg().to_tcod(),
        ce.get_bg().to_tcod()
      );
    }

  }

  ///
  /// Draw the log
  ///
  /// Currently still testing
  ///
  pub fn draw_log(&self, con: &mut console::Root) {

    // Mutable reference to the mutex
    let log = GlobalLog.lock().unwrap();

    // Iterage over the latest range
    for i in log.get_latest_range(self.console_height as usize) {
      // Draw to screen
      let y = self.screen.y - ((log.data.len() as isize) - i as isize);
      let color = log.data[i].1;
      let s = log.data[i].0;
      con.set_default_foreground(color.to_tcod());
      con.print(0, y as i32, s);
    }

    // Test mutability
    // log.push(("Update", RGB(255, 255, 255)));

    drop(log);

  }

  ///
  /// Return a new `Renderer`
  ///
  /// * `map` - `Pos` that holds the map dimensions
  /// * `screen` - `Pos` that holds the screen dimensions
  ///
  #[inline]
  pub fn new(map: (isize, isize), screen: (isize, isize), console_height: isize, panel_width: isize) -> Renderer {
    Renderer { 
      // Camera takes a modified screen value that compensates for the console_height
      // This way the render still knows that that area is "reserved" for the console
      camera: Camera::new(map, (screen.0 - panel_width - 1, screen.1 - console_height - 1)), 
      console_height: console_height, panel_width: panel_width,
      screen: Pos::from_tup(screen),
      sc_debug: false, fov: true, so_debug: false 
    }
  }

}
