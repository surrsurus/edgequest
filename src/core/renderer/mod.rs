//!
//! Metapackage for renderer
//! 

pub mod camera;

pub use self::camera::Camera;

use core::Game;

use core::object::{Pos, Entity};

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
  // Map dimensions
  map: Pos,
  // Screen dimensions
  screen: Pos,
}

impl Renderer {

  ///
  /// Draw all.
  /// 
  /// You'll have to render this console to root (unless you passed root in)
  /// and always `flush()` the root console.
  /// 
  pub fn draw_all(&mut self, con: &mut Console, game: &Game) {

    println!("{}", &game.player.glyph);

    // Clear console
    con.clear();

    self.camera.move_to(game.player.pos);

    // Draw tiles
    for t in game.floor.tile_vec.iter() { self.draw_entity(con, &t.entity); }

    // Draw entities
    for e in game.floor.entity_vec.iter() { self.draw_entity(con, e); }

    // Draw player. Player is always in the camera since
    // we move the camera over it.
    self.draw_entity(con, &game.player);

  }

  ///
  /// Put an `Entity` on the console
  /// 
  /// * `con` - Tcod `Console`
  /// * `entity` - `Entity`
  /// 
  pub fn draw_entity(&self, con: &mut Console, entity: &Entity) {

    // Check if it's in the camera first
    if !self.camera.is_in_camera(entity.pos) { return }

    // New pos with respect to camera
    let pos = entity.pos + self.camera.pos;

    if entity.glyph == ' ' {
      con.set_char_background(
        pos.x,
        pos.y,
        entity.bg.to_tcod_color(),
        console::BackgroundFlag::Set
      );
    } else {
      con.put_char_ex(
        pos.x, 
        pos.y, 
        entity.glyph,
        entity.fg.to_tcod_color(),
        entity.bg.to_tcod_color()
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
    Renderer { camera: Camera::new(map, screen), map: map, screen: screen }
  }

}