//!
//! Metapackage for renderer
//! 

pub mod camera;

pub use self::camera::Camera;

use core::Game;

use core::dungeon::map::Tile;

use core::object::{Pos, RenderableEntity, RGB};

use core::tcod::{Console, console};

pub trait Screen {

  fn draw_all(&mut self, con: Console, game: &Game);

}

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
}

impl Renderer {

  pub fn debug_render_scent_map(&mut self, con: &mut Console, game: &Game) {

    for x in 0..game.dungeon.width {
      for y in 0..game.dungeon.height {
        if game.dungeon.scent_map.0[x][y].value != 0 {
          let b : u8;
          if game.dungeon.scent_map.0[x][y].value < 30 {
            b = 70 + game.dungeon.scent_map.0[x][y].value * 6;
          } else {
            b = 100;
          }
          self.draw_entity(con, Pos::new(x as isize, y as isize), &Tile::new(
            "Debug Scent".to_string(),
            ' ',
            RGB(255, 255, 255),
            RGB(game.dungeon.scent_map.0[x][y].value, b / 2, b),
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
  pub fn draw_all(&mut self, con: &mut Console, game: &Game) {

    // Clear console
    con.clear();

    self.camera.move_to(game.player.pos);

    // Draw tiles
    for x in 0..game.dungeon.width {

      for y in 0..game.dungeon.height {
        
        self.draw_entity(con, Pos::new(x as isize, y as isize), &game.dungeon.grid.0[x][y]);

      }

    }

    // Debug
    self.debug_render_scent_map(con, game);

    // Draw player. Player is always in the camera since
    // we move the camera over it.
    self.draw_entity(con, game.player.pos, &game.player);

  }

  ///
  /// Put an `Entity` on the console
  /// 
  /// * `con` - Tcod `Console`
  /// * `entity` - `Entity`
  /// 
  pub fn draw_entity(&self, con: &mut Console, pos: Pos, ce: &RenderableEntity) {
    
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
  pub fn new(map: Pos, screen: Pos) -> Renderer {
    Renderer { camera: Camera::new(map, screen) }
  }

}