//!
//! Metapackage for renderer
//! 

pub mod camera;
pub use self::camera::Camera;

use core::world::World;

use core::world::dungeon::Dungeon;

use core::world::dungeon::map::tile;
use core::world::dungeon::map::{Tile, TileType, SCENT_TYPES};

use core::object::{RGB, Pos, Entity};

use core::tcod::{Console, console};

extern crate rand;
use self::rand::{thread_rng, Rng};

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
        let mut color : (u8, u8, u8) = (dungeon.grid[x][y].scents[0].val + 50, dungeon.grid[x][y].scents[1].val + 25, dungeon.grid[x][y].scents[2].val + 50 );
        for s in 0..SCENT_TYPES {
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
  /// Draw all.
  /// 
  /// You'll have to render this console to root (unless you passed root in)
  /// and always `flush()` the root console.
  /// 
  pub fn draw_all(&mut self, con: &mut Console, world: &mut World) {

    // Clear console
    con.clear();

    self.camera.move_to(world.player.pos);

    let mut rng = thread_rng();

    // Draw seen tiles
    for x in 0..world.cur_dungeon.width {
      for y in 0..world.cur_dungeon.height {
        // If fov is on...
        if self.fov {
          // And it's in the FOV
          if world.tcod_map.is_in_fov(x as i32, y as i32) && self.fov {
            match world.cur_dungeon.grid[x][y].tiletype {
              TileType::Water => {

                let mut t = world.cur_dungeon.grid[x][y].clone();
                
                match rng.gen_range(1, 11) as i32 {
                  1 => {t = t.amplify_col(RGB(0, 0, 5));}
                  2 => {t = t.amplify_col(RGB(0, 0, 10));}
                  3 => {t = t.amplify_col(RGB(0, 0, 15));}
                  4 => {t = t.amplify_col(RGB(0, 0, 20));}
                  5 => {t = t.amplify_col(RGB(0, 2, 5));}
                  6 => {t = t.amplify_col(RGB(0, 2, 10));}
                  7 => {t = t.amplify_col(RGB(0, 2, 15));}
                  8 => {t = t.amplify_col(RGB(0, 2, 20));}
                  9 => {t = t.amplify_col(RGB(0, 2, 15));}
                  10 => {t = t.amplify_col(RGB(0, 2, 10));}
                  _ => unreachable!("whoopie looks like we made a fucky wucky")
                }

                self.draw_entity(con, Pos::new(x as isize, y as isize), &t);

              },
              _ => {
                self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y].yellowish());
              }
            }
            
            world.cur_dungeon.grid[x][y].seen = true;
          }
          // And the tile has been seen...
          else if world.cur_dungeon.grid[x][y].seen {
            // Draw certain tiles depending on their types
            match world.cur_dungeon.grid[x][y].tiletype {
              _ => {
                self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y].darken());
              }
            }
          }
        } 
        // Debug just draw all tiles normally
        else { 
         match world.cur_dungeon.grid[x][y].tiletype {
            _ => {
              self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y]);
            }
          }
        }
      }
    }

    // Debug
    if self.sc_debug {
      self.debug_render_scent_map(con, &world.cur_dungeon);
    }

    for c in &world.creatures {
      // If fov is on...
      if self.fov {
        // And its in the fov...
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
  pub fn draw_creature(&self, con: &mut Console, pos: Pos, ce: &Entity, world: &World) {
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