//!
//! Metapackage for renderer
//! 

pub mod camera;
pub use self::camera::Camera;

use core::world::World;

use core::world::dungeon::Dungeon;

use core::world::dungeon::map::{Tile, TileType};

use core::object::{RGB, Pos, Entity};

use core::tcod::{Console, console};

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
  pub sc_debug: bool,
  pub fov: bool
}

impl Renderer {

  ///
  /// Render scent as a fine, purple mist, just like real life
  ///
  pub fn debug_render_scent_map(&mut self, con: &mut Console, dungeon: &Dungeon) {

    for x in 0..dungeon.width {
      for y in 0..dungeon.height {
        if dungeon.grid[x][y].scent > 0 {
          self.draw_entity(con, Pos::new(x as isize, y as isize), &Tile::new(
            "Debug Scent",
            ' ',
            (255, 255, 255),
            (dungeon.grid[x][y].scent + 50, 0, dungeon.grid[x][y].scent + 25),
            TileType::Debug
          ));
        }
      }
    }

  }

  ///
  /// Draw all.
  /// 
  /// You'll have to render this console to root (unless you passed root in)
  /// and always `flush()` the root console.
  /// 
  pub fn draw_all(&mut self, con: &mut Console, world: &mut World) {

    // Clear console
    con.clear();

    self.camera.move_to(world.player.pos);

    // Draw seen tiles. Spaghetti, needs to change
    for x in 0..world.cur_dungeon.width {
      for y in 0..world.cur_dungeon.height {
        if self.fov {
          if world.cur_dungeon.grid[x][y].seen {
            match world.cur_dungeon.grid[x][y].tiletype {
                TileType::Wall => {
                  self.draw_entity(con, Pos::new(x as isize, y as isize), &Tile::new(
                    "Seen Wall",
                    ' ',
                    (0, 0, 0),
                    (70, 70, 70),
                    TileType::Unseen
                  ));
                }
                _ => {
                  self.draw_entity(con, Pos::new(x as isize, y as isize), &Tile::new(
                    "Seen Floor",
                    ' ',
                    (0, 0, 0),
                    (40, 40, 40),
                    TileType::Unseen
                  ));
                },
            }
             
          }
        }
      }
    }

    // Draw tiles
    for x in 0..world.cur_dungeon.width {
      for y in 0..world.cur_dungeon.height {
        if self.fov {
          if world.tcod_map.is_in_fov(x as i32, y as i32) && self.fov {
            self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y]);
            world.cur_dungeon.grid[x][y].seen = true;
          }
        } else {
          self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y]);
        }
        match world.cur_dungeon.grid[x][y].tiletype {
          TileType::UpStair => {
            self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y]);
          },
          TileType::DownStair => {
            self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y]);
          },
          _ => {}
        }
      }
    }

    // Debug
    if self.sc_debug {
      self.debug_render_scent_map(con, &world.cur_dungeon);
    }

    for c in &world.creatures {
      if self.fov {
        if world.tcod_map.is_in_fov(c.fighter.pos.x as i32, c.fighter.pos.y as i32) && self.fov {
          self.draw_creature(con, c.fighter.pos, &c.fighter, world);
        }
      } else {
        self.draw_creature(con, c.fighter.pos, &c.fighter, world);
      }
    }

    // Draw player. Player is always in the camera since
    // we move the camera over it.
    self.draw_creature(con, world.player.pos, &world.player, world);

  }

  ///
  /// Draw creatures with "transparent" backgrounds
  ///
  /// NOTE: Way too similar to draw_entity(). Plus, the whole bg thing seems weird.
  /// Better way to do it?
  ///
  pub fn draw_creature(&self, con: &mut Console, pos: Pos, ce: &Entity, world: &World) {
    // Check if it's in the camera first
    if !self.camera.is_in_camera(pos) { return }

    // New pos with respect to camera
    let npos = pos + self.camera.pos;

    if ce.get_glyph() == ' ' {
      con.set_char_background(
        npos.x as i32,
        npos.y as i32,
        ce.get_bg().to_tcod(),
        console::BackgroundFlag::Set
      );
    } else {
      let bg : RGB;
      if ce.get_bg() == RGB(0, 0, 0) {
        bg = world.get_bg_color_at(pos.x as usize, pos.y as usize);
      } else {
        bg = ce.get_bg()
      }
      con.put_char_ex(
        npos.x as i32, 
        npos.y as i32, 
        ce.get_glyph(),
        ce.get_fg().to_tcod(),
        bg.to_tcod()
      );
    }
  }

  ///
  /// Put an `Entity` on the console
  /// 
  /// * `con` - Tcod `Console`
  /// * `entity` - `Entity`
  /// 
  pub fn draw_entity(&self, con: &mut Console, pos: Pos, ce: &Entity) {
    
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
  /// Return a new `Renderer`
  /// 
  /// * `map` - `Pos` that holds the map dimensions
  /// * `screen` - `Pos` that holds the screen dimensions
  /// 
  #[inline]
  pub fn new(map: (isize, isize), screen: (isize, isize)) -> Renderer {
    Renderer { camera: Camera::new(map, screen), sc_debug: false, fov: true }
  }

}