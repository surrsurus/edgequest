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

mod renderable;
pub use self::renderable::Renderable;

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
fn amplify_col(tile: &Tile, factor: RGB) -> Tile {
  let mut replace = tile.clone();
  replace.fg = tile.fg + factor;
  replace.bg = tile.bg + factor;
  return replace;
}

///
/// Modify a tile's fg and bg color
///
fn reduce_col(tile: &Tile, factor: RGB) -> Tile {
  let mut replace = tile.clone();
  replace.fg = tile.fg - factor;
  replace.bg = tile.bg - factor;
  return replace;
}

///
/// Darken a tile's fg and bg color
///
fn darken(tile: &Tile) -> Tile {
  reduce_col(tile, DARKEN_FAC)
}

///
/// Make a tile's fg and bg color more yellowish
///
fn yellowish(tile: &Tile) -> Tile {
  amplify_col(tile, YELLOW_FAC)
}

///
/// The renderer
///
/// Tracks the player and automatically scrolls the screen around to match where they go.
/// This will never try to draw things outside of the given dimensions due to the way it handles
/// determining whether something should be drawn or not.
///
pub struct Renderer {
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
        for scent_type in 0..tile::Scent::Num as usize {
          if dungeon[x][y].scents[scent_type].val > 0 {
            self.draw_renderable(con, Pos::new(x as isize, y as isize), &Tile::new(
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
          self.draw_renderable(con, Pos::new(x as isize, y as isize), &Tile::new(
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
  /// Draw the log
  ///
  /// We need to directly manipulate the GlobalLog object so here we use the mutex lock
  ///
  fn draw_log(&self, con: &mut console::Root) {

    // Mutable reference to the mutex
    let log = GlobalLog.lock().unwrap();

    // Enumerate over the last few messages
    for (idx, pair) in log.get_last_n_messages(self.console_height as usize).iter().enumerate() {
      // Y value of text is determined by the index
      let y = self.screen.y - ((log.get_last_n_messages(self.console_height as usize).len() as isize) - idx as isize);
      // Color and string is determined by the content of the slice at that index
      let color = pair.1;
      let string = pair.0;
      // They are then combined to render to the screen at a specific y axis such that the most
      // recent message will appear at the bottom
      con.set_default_foreground(color.to_tcod());
      con.print(0, y as i32, string);
    }

    // Explicitly drop ref
    drop(log);

  }
  
  ///
  /// Put an `Renderable` on the console
  ///
  fn draw_renderable(&self, con: &mut console::Root, pos: Pos, renderable: &Renderable) {

    // Check if it's in the camera first
    if !self.camera.is_in_camera(pos) { return }

    // New pos with respect to camera
    let pos = pos + self.camera.pos;

    con.put_char_ex(
      pos.x as i32,
      pos.y as i32,
      renderable.get_glyph(),
      renderable.get_fg().to_tcod(),
      renderable.get_bg().to_tcod()
    );

  }

  ///
  /// Draw renderables with "transparent" backgrounds
  ///
  fn draw_renderable_transparent(&self, con: &mut console::Root, pos: Pos, renderable: &Renderable, world: &World) {
    // Check if it's in the camera first
    if !self.camera.is_in_camera(pos) { return }

    // New pos with respect to camera
    let npos = pos + self.camera.pos;
  
    con.put_char_ex(
      npos.x as i32,
      npos.y as i32,
      renderable.get_glyph(),
      renderable.get_fg().to_tcod(),
      // Backgrounds are just inherited from the world.
      if self.fov {
        (world.get_bg_color_at(pos) + YELLOW_FAC).to_tcod()
      } else {
        (world.get_bg_color_at(pos)).to_tcod()
      }
    );

  }

  ///
  /// Draw UI elements
  /// 
  /// NOTE: This function is super basic and is intended to be revised/removed
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
    let mut non_player_scent = 0;
    for scent in &tile.scents {
      if &scent.scent_type != &tile::Scent::Player { 
        non_player_scent += scent.val; // BUG: Panics here with overflow
      }
    }

    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      5,
      format!("{}: {}", "Non-player Scent", non_player_scent)
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
      format!("{}: {}", "Tile", tile.get_id())
    );

    // Floor number
    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      8,
      format!("{}: {}", "Floor", world.floor_num)
    );

    // Wallet
    con.print(
      (self.screen.x - self.panel_width + 1) as i32,
      9,
      format!("{}: {}", "AU", world.player.wallet)
    );

  }

  ///
  /// Draw the contents of the world from the player's point of view
  /// 
  fn draw_world(&mut self, con: &mut console::Root, world: &mut World) {

    // Draw the world in three steps:
    //
    //  1. Place camera on player
    //  2. Draw tiles
    //  3. Draw debug information (I want to move this out... if its not here this fn doesn't need to be &mut self)
    //  (Drawing items should go here)
    //  4. Draw creatures

    // Move camera to player's position
    self.camera.move_to(world.player.actor.pos);

    //
    // Draw tiles
    //

    // Draw seen tiles
    for x in 0..world.floor.dun.width {
      for y in 0..world.floor.dun.height {
        // If fov is on...
        if self.fov {
          // And it's in the FoV
          if world.tcod_map.is_in_fov(x as i32, y as i32) {

            // Update tile if possible
            match &world.floor.dun[x][y].tiletype {
              tile::Type::Water => {
                &world.floor.dun[x][y].set_bg(*rand::thread_rng().choose(&WATER_COLORS).unwrap());
              },
              _ => {}
            }

            // Draw a tile slightly more vibrant than it actually is to emulate torchlight
            self.draw_renderable(con, Pos::from_usize(x, y), &yellowish(&world.floor.dun[x][y]));

            // Mark tile as seen if it's in the FoV
            world.floor.dun[x][y].seen = true;

          }

          // And the tile has been seen...
          else if world.floor.dun[x][y].seen {
            // Draw a tile, but darker
            self.draw_renderable(con, Pos::from_usize(x, y), &darken(&world.floor.dun[x][y]));
          }

        }

        // [Debug] Otherwise just draw all tiles normally
        else {
          self.draw_renderable(con, Pos::new(x as isize, y as isize), &world.floor.dun[x][y]);
        }
        
      }
    }

    // Draw items
    for item in &world.floor.items {
      // If fov is on...
      if self.fov {
        // And it's in the FoV
        if world.tcod_map.is_in_fov(item.pos.x as i32, item.pos.y as i32) {
          self.draw_renderable_transparent(con, item.pos, item, world);
        }
      } 
      // [Debug] Otherwise just draw all tiles normally
      else {
        self.draw_renderable(con, item.pos, item);
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

    for creature in &world.floor.creatures {
      // If fov is on...
      if self.fov {
        // And its in the fov...
        if world.tcod_map.is_in_fov(creature.actor.pos.x as i32, creature.actor.pos.y as i32) {
          self.draw_renderable_transparent(con, creature.actor.pos, &creature.actor, world);
        }
      } else {
        self.draw_renderable_transparent(con, creature.actor.pos, &creature.actor, world);
      }
    }

    // Draw player. Player is always in the camera since
    // we move the camera over it.
    self.draw_renderable_transparent(con, world.player.actor.pos, &world.player.actor, world);

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
  pub fn new(map: Pos, screen: Pos, console_height: isize, panel_width: isize) -> Self {
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
