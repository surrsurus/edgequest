//!
//! Metapackage for renderer
//! 

pub mod camera;
pub use self::camera::Camera;

use core::world::World;

use core::dungeon::Dungeon;

use core::dungeon::map::Tile;

use core::object::{Pos, Entity};

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
  pub debug: bool
}

impl Renderer {

  pub fn debug_render_scent_map(&mut self, con: &mut Console, dungeon: &Dungeon) {

    for x in 0..dungeon.width {
      for y in 0..dungeon.height {
        if dungeon.grid[x][y].scent > 0 {
          self.draw_entity(con, Pos::new(x as isize, y as isize), &Tile::new(
            "Debug Scent".to_string(),
            ' ',
            (255, 255, 255),
            (dungeon.grid[x][y].scent + 50, 0, dungeon.grid[x][y].scent + 25),
            false
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
  pub fn draw_all(&mut self, con: &mut Console, world: &World) {

    // Clear console
    con.clear();

    self.camera.move_to(world.player.pos);

    // Draw tiles
    for x in 0..world.cur_dungeon.width {
      for y in 0..world.cur_dungeon.height {
        self.draw_entity(con, Pos::new(x as isize, y as isize), &world.cur_dungeon.grid[x][y]);
      }
    }

    // Debug
    if self.debug {
      self.debug_render_scent_map(con, &world.cur_dungeon);
    }

    for c in &world.creatures {
      self.draw_entity(con, c.fighter.pos, &c.fighter);
    }

    // Draw player. Player is always in the camera since
    // we move the camera over it.
    self.draw_entity(con, world.player.pos, &world.player);

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
    Renderer { camera: Camera::new(map, screen), debug: false }
  }

}