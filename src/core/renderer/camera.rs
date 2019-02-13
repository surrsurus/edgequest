//!
//! Select specific parts of the screen to be rendered
//!

use super::Pos;

///
/// `Camera` struct. A camera simply holds a position (Where it is looking),
/// and holds information about the map size and screen size (Held as `Pos`s)
///
/// Note that the map size can be less than the screen size and this will still work fine,
/// Although the `Camera` likes to place emphasis on the bottom right corner of the screen,
/// as that is where the boundary often extends beyond and special care must be taken.
/// 
/// Theoretically, the camera does not need tcod to function, and should work for any terminal or tile based
/// renderer.
///
pub struct Camera {
  // Position that the camera is panned to on the map
  // Must be within map bounds, or camera will just go to the region,
  // though the target won't be exactly in the center of the screen.
  pub pos: Pos,

  // Map dimensions
  map: Pos,
  
  // Screen dimensions
  screen: Pos,

}

impl Camera {

  ///
  /// Check if a `Pos` is in the camera. Used to determine if something should be rendered or not.
  /// 
  #[inline]
  pub fn is_in_camera(&self, pos: Pos) -> bool {
    // New pos to compare things to without totally cluttering the function
    let npos = pos + self.pos;
    if npos.x >= 0 && npos.x < self.screen.x && npos.y >= 0 && npos.y < (self.screen.y) { true } else { false }
  }

  ///
  /// Move camera over a position on the map. Used to center on the player or points of interest.
  /// 
  /// The camera will prevent itself from going OOB.
  /// 
  pub fn move_to(&mut self, pos: Pos) {

    // Copy position
    let mut new_pos = pos.clone();

    // We want to be somewhere in the middle of the map, but judge based on the max
    // bounds of the screen. This is what pushes the camera to the bottom right of the screen
    new_pos -= Pos::new(self.screen.x / 2, (self.screen.y) / 2);

    // Boundary checks
    if new_pos.x < 0 { new_pos.x = 0; }
    if new_pos.y < 0 { new_pos.y = 0; }
    if new_pos.x > self.map.x - self.screen.x { new_pos.x = self.map.x - self.screen.x; }
    if new_pos.y > self.map.y - self.screen.y { new_pos.y = self.map.y - self.screen.y; }

    // Some cool math gets us a good position to be at
    self.pos = -new_pos;

  }

  ///
  /// Return a new `Camera`
  /// 
  /// * `map` - `Pos` that holds the map dimensions
  /// * `screen` - `Pos` that holds the screen dimensions
  /// 
  #[inline]
  pub fn new(map: Pos, screen: Pos) -> Self {
    Camera { pos: Pos::origin(), map: map, screen: screen}
  }

}