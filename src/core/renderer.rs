use core::tcod::colors;
use core::tcod::Console;
use core::tcod::console;
use core::object::pos::Pos;
use core::object::rgb::RGB;
use core::object::entity::Entity;
use core::Game;

///
/// Helper for rendering things to the screen
/// 
/// Tracks the player and automatically scrolls the screen around to match where they go.
/// This will never try to draw things outside of the given dimensions due to the way it handles
/// determining whether something should be drawn or not. 
/// 
pub struct Renderer {

  // Position that the camera is panned to on the map
  // Must be within map bounds, or camera will just go to the region,
  // though the target won't be exactly in the center of the screen.
  camera: Pos,

  // Map dimensions
  map: Pos,

  // Screen dimensions
  screen: Pos,

}

fn to_tcod_color(rgb: RGB) -> colors::Color {
  return colors::Color::new(rgb.0, rgb.1, rgb.2)
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

    self.move_to(game.player.pos);

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
    if !self.is_in_camera(entity.pos) {
      return;
    }

    // New pos with respect to camera
    let pos = entity.pos + self.camera;

    if entity.glyph == ' ' {
      con.set_char_background(
        pos.x,
        pos.y,
        entity.get_bg(),
        console::BackgroundFlag::Set
      );
    } else {
      con.put_char_ex(
        pos.x, 
        pos.y, 
        entity.glyph,
        entity.get_fg(),
        entity.get_bg()
      );
    }

  }

  ///
  /// Check if a `Pos` is in the camera
  /// 
  #[inline]
  pub fn is_in_camera(&self, pos: Pos) -> bool {

    // New pos to compare things to
    let npos = pos + self.camera;

    if npos.x >= 0 && npos.x < self.screen.x && npos.y >= 0 && npos.y < self.screen.y { return true; } else { return false; };

  }

  ///
  /// Move camera over a position on the map
  /// 
  /// The camera will prevent itself from going OOB.
  /// 
  fn move_to(&mut self, pos: Pos) {

    let mut x = pos.x - (self.screen.x / 2);
    let mut y = pos.y - (self.screen.y / 2);

    if x < 0 { x = 0; }
    if y < 0 { y = 0; }
    if x > self.map.x - self.screen.x - 1 { x = self.map.x - self.screen.x - 1; }
    if y > self.map.y - self.screen.y - 1 { y = self.map.y - self.screen.y - 1; }

    self.camera = Pos::new(-x, -y);

  }

  ///
  /// Return a new `Renderer`
  /// 
  /// * `map` - `Pos` that holds the map dimensions
  /// * `screen` - `Pos` that holds the screen dimensions
  /// 
  pub fn new(map: Pos, screen: Pos) -> Renderer {
    return Renderer { camera: Pos::origin(), map: map, screen: screen };
  }

}