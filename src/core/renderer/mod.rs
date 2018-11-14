//!
//! Metapackage for renderer
//!

extern crate rand;
use self::rand::Rng;

// Convert numbers to chars
use std::char;

// `Console` is needed as Console is a trait that console::Root extends
use core::tcod::{Console, console};

use core::GlobalLog;

use core::world::World;

use core::world::dungeon::Dungeon;
// Used to expliclty reference constants
use core::world::dungeon::map::{tile, Pos, Tile};

mod entity;
pub use self::entity::Entity;

// Use camera privately
mod camera;
use self::camera::Camera;

// Use RGB publicly
pub mod rgb;
pub use self::rgb::RGB;

///
/// Tile colors
///

// Water colors
const WATER_COLORS : [RGB; 3] = [
  RGB(51, 133, 200),
  RGB(57, 144, 200),
  RGB(54, 138, 200)
];

///
/// Tile color manipulation
/// 

// Used to darken tiles that are out of sight
pub const DARKEN_FAC : RGB = RGB(10, 10, 10);
// Used to lighten tiles that are in the FoV
pub const YELLOW_FAC : RGB = RGB(27, 24, 22);

///
/// Modify a tile's fg and bg color
///
fn amplify_col(t: &Tile, factor: RGB) -> Tile {
  let mut replace = t.clone();
  replace.fg = t.fg + factor;
  replace.bg = t.bg + factor;
  return replace;
}

///
/// Modify a tile's fg and bg color
///
fn reduce_col(t: &Tile, factor: RGB) -> Tile {
  let mut replace = t.clone();
  replace.fg = t.fg - factor;
  replace.bg = t.bg - factor;
  return replace;
}

///
/// Darken a tile's fg and bg color
///
fn darken(t: &Tile) -> Tile {
  reduce_col(t, DARKEN_FAC)
}

///
/// Make a tile's fg and bg color more yellowish
///
fn yellowish(t: &Tile) -> Tile {
  amplify_col(t, YELLOW_FAC)
}

///
/// The renderer
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
  pub show_scent: bool,
  pub fov: bool,
  pub show_sound: bool
}

impl Renderer {

  ///
  /// Render for each monster as a visible colored entity
  ///
  fn debug_render_scent_map(&mut self, con: &mut console::Root, dungeon: &Dungeon) {

    for x in 0..dungeon.width {
      for y in 0..dungeon.height {
        // Pretty much just random, Player is red, bugs are green, cats are yellow and dogs are blue
        let mut color = RGB(
          dungeon[x][y].scents[0].val + 50 + dungeon[x][y].scents[3].val, 
          dungeon[x][y].scents[1].val + 25 + dungeon[x][y].scents[3].val, 
          dungeon[x][y].scents[2].val + 50 
        );
        // Iterate over scents, context of what scent it is isn't necessary
        for s in 0..tile::Scent::Num as usize {
          if dungeon[x][y].scents[s].val > 0 {
            self.draw_entity(con, Pos::new(x as isize, y as isize), &Tile::new(
              "Debug Scent",
              ' ',
              RGB(255, 255, 255),
              color,
              tile::Type::Debug
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
  fn debug_render_sound_map(&mut self, con: &mut console::Root, dungeon: &Dungeon) {

    for x in 0..dungeon.width {
      for y in 0..dungeon.height {
        // Color is weighted towards blue
        let mut color = RGB(
          dungeon[x][y].get_bg().r(), 
          dungeon[x][y].get_bg().g(), 
          dungeon[x][y].sound as u8
        );
        if dungeon[x][y].sound > 0 {
          self.draw_entity(con, Pos::new(x as isize, y as isize), &Tile::new(
            "Debug Sound",
            ' ',
            RGB(255, 255, 255),
            color,
            tile::Type::Debug
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
    // Draw world
    //

    self.draw_world(con, world);

    //
    // Draw log
    //

    self.draw_log(con);

    //
    // Draw UI
    //
    
    self.draw_ui(con, world);


    //
    // Flush changes to root
    //

    con.flush();

  }

  ///
  /// Draw creatures with "transparent" backgrounds
  ///
  fn draw_creature(&self, con: &mut console::Root, pos: Pos, ce: &Entity, world: &World) {
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
        (world.get_bg_color_at(pos) + YELLOW_FAC).to_tcod()
      } else {
        (world.get_bg_color_at(pos)).to_tcod()
      }
    );

  }

  ///
  /// Put an `Entity` on the console
  ///
  fn draw_entity(&self, con: &mut console::Root, pos: Pos, ce: &Entity) {

    // Check if it's in the camera first
    if !self.camera.is_in_camera(pos) { return }

    // New pos with respect to camera
    let pos = pos + self.camera.pos;

    con.put_char_ex(
      pos.x as i32,
      pos.y as i32,
      ce.get_glyph(),
      ce.get_fg().to_tcod(),
      ce.get_bg().to_tcod()
    );

  }

  ///
  /// Draw the log
  ///
  /// We need to directly manipulate the GlobalLog object so here we use the mutex lock
  ///
  fn draw_log(&self, con: &mut console::Root) {

    // Mutable reference to the mutex
    let log = GlobalLog.lock().unwrap();

    // Enumerate over the last few messages
    for (i, pair) in log.get_latest_range(self.console_height as usize).iter().enumerate() {
      // Y value of text is determined by the index
      let y = self.screen.y - ((log.get_latest_range(self.console_height as usize).len() as isize) - i as isize);
      // Color and string is determined by the content of the slice at that index
      let color = pair.1;
      let s = pair.0;
      // They are then combined to render to the screen at a specific y axis such that the most
      // recent message will appear at the bottom
      con.set_default_foreground(color.to_tcod());
      con.print(0, y as i32, s);
    }

    drop(log);

  }
  
  ///
  /// Draw UI elements
  /// 
  /// NOTE: This function is super basic and is intended to be revised/removed.Option
  /// 
  fn draw_ui(&self, con: &mut console::Root, world: &mut World) {
    
    // Draw horizontal line to split game from the log console
    for x in 0..self.screen.x {
      con.put_char_ex(
        x as i32,
        (self.screen.y - self.console_height - 1) as i32,
        '-',
        RGB(255, 255, 255).to_tcod(),
        RGB(0, 0, 0).to_tcod()
      );
    }

    // Draw horizontal line to split game from the panel
    for y in 0..(self.screen.y - self.console_height - 1) {
      con.put_char_ex(
        (self.screen.x - self.panel_width - 1) as i32,
        y as i32,
        '|',
        RGB(255, 255, 255).to_tcod(),
        RGB(0, 0, 0).to_tcod()
      );
    }

    // Pretty sure this places a piece at the intersection of the panel and console lines
    // but god damn
    con.put_char_ex(
      (self.screen.x - self.panel_width - 1) as i32,
      (self.screen.y - self.console_height - 1) as i32,
      // hyperthonk
      char::from_u32(193).unwrap(),
      RGB(255, 255, 255).to_tcod(),
      RGB(0, 0, 0).to_tcod()
    );

    // Tile player is on
    let tile = &world.floor.dun[world.player.actor.pos];

    //
    //  Draw side panel contents
    // 
    
    // White on black because I'm lazy
    con.set_default_foreground(RGB(255, 255, 255).to_tcod());

    // Gotta remind people what game theyre playing
    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      1,
      "Edgequest"
    );

    // Paying my respects to a legend
    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      2,
      "This is where we live"
    );

    // Biome
    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      4,
      format!("{}: {}", "Biome", tile.biome)
    );

    // Scent of non-players
    let mut npscent = 0;
    for s in &tile.scents {
      if &s.scent_type != &tile::Scent::Player { 
        npscent += s.val; // BUG: Panics here with overflow
      }
    }

    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      5,
      format!("{}: {}", "Non-player Scent", npscent)
    );

    // Sound
    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      6,
      format!("{}: {}", "Sound", tile.sound)
    );

    // Tile
    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      7,
      format!("{}: {}", "Tile", tile.get_name())
    );

    // Floor number
    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      8,
      format!("{}: {}", "Floor", world.floor_num)
    );
  }

  ///
  /// Draw the contents of the world from the player's point of view
  /// 
  fn draw_world(&mut self, con: &mut console::Root, world: &mut World) {

    // Draw the world in three steps:
    //
    //  1. Tiles
    //  2. Debug information (I want to move this out... if its not here this fn doesn't need to be &mut self)
    //  3. Creatures

    //
    // Draw tiles
    //

    // Draw seen tiles
    for x in 0..world.floor.dun.width {
      for y in 0..world.floor.dun.height {
        // If fov is on...
        if self.fov {
          // And it's in the FoV
          if world.tcod_map.is_in_fov(x as i32, y as i32) && self.fov {

            // Update tile if possible
            match &world.floor.dun[x][y].tiletype {
              tile::Type::Water => {
                &world.floor.dun[x][y].set_bg(*rand::thread_rng().choose(&WATER_COLORS).unwrap());
              },
              _ => {}
            }

            // Draw a tile slightly more vibrant than it actually is to emulate torchlight
            self.draw_entity(con, Pos::from_usize(x, y), &yellowish(&world.floor.dun[x][y]));

            // Mark tile as seen if it's in the FoV
            world.floor.dun[x][y].seen = true;

          }

          // And the tile has been seen...
          else if world.floor.dun[x][y].seen {
            // Draw a tile, but darker
            self.draw_entity(con, Pos::from_usize(x, y), &darken(&world.floor.dun[x][y]));
          }

        }

        // [Debug] Otherwise just draw all tiles normally
        else {
          self.draw_entity(con, Pos::new(x as isize, y as isize), &world.floor.dun[x][y]);
        }
        
      }
    }

    //
    // Debug options
    //

    // We need the Renderer to be &mut here because of the way the Renderer is kept in context

    // Debug scent
    if self.show_scent {
      self.debug_render_scent_map(con, &world.floor.dun);
    }

    // Debug sound
    if self.show_sound {
      self.debug_render_sound_map(con, &world.floor.dun);
    }

    //
    // Draw creatures
    //

    for c in &world.floor.creatures {
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

  }

  ///
  /// Print all renderable characters in the font
  /// 
  pub fn tcod_test(&self, con: &mut console::Root) {

    let w = con.width();
    let h = con.height();

    // Clear screen
    for x in 0..w {
      for y in 0..h {
        con.put_char_ex(
          x as i32,
          y as i32,
          ' ',
          RGB(0, 0, 0).to_tcod(),
          RGB(0, 0, 0).to_tcod()
        );
      }
    }

    // Print all 2^8 characters
    for ord in 0..256 {
      con.put_char_ex(
        (ord % w) as i32,
        (ord / w) as i32,
        // Basically go from ascii to a char
        char::from_u32(ord as u32).unwrap(),
        RGB(255, 255, 255).to_tcod(),
        RGB(0, 0, 0).to_tcod()
      );
    }

    // Update console
    con.flush();

    // Wait for keypress
    con.wait_for_keypress(true);

  }

  ///
  /// Return a new `Renderer`
  ///
  /// * `map` - `Pos` that holds the map dimensions
  /// * `screen` - `Pos` that holds the screen dimensions
  ///
  #[inline]
  pub fn new(map: Pos, screen: Pos, console_height: isize, panel_width: isize) -> Renderer {
    Renderer { 
      // Camera takes a modified screen value that compensates for the console_height
      // This way the render still knows that that area is "reserved" for the console
      camera: Camera::new(
        map, 
        Pos::new(screen.x - panel_width - 1, screen.y - console_height - 1)
      ), 
      console_height: console_height, panel_width: panel_width,
      screen: screen,
      show_scent: false, fov: true, show_sound: false 
    }
  }

}
