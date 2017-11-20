//!
//! Metapackage for renderer
//! 

pub mod camera;

pub use self::camera::Camera;

use core::Game;

use core::object::{Pos, RenderableEntity};

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
    for x in 0..game.floor.tile_vec.0.len() {

      for y in 0..game.floor.tile_vec.0[0].len() {
        
        self.draw_entity(con, Pos::new(x as isize, y as isize), &game.floor.tile_vec.0[x][y]);

      }

    }

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