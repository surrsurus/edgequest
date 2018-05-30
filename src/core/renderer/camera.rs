use core::object::Pos;

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
  /// Check if a `Pos` is in the camera
  /// 
  #[inline]
  pub fn is_in_camera(&self, pos: Pos) -> bool {
    // New pos to compare things to without totally cluttering the function
    let npos = pos + self.pos;
    if npos.x >= 0 && npos.x < self.screen.x && npos.y >= 0 && npos.y < (self.screen.y) { true } else { false }
  }

  ///
  /// Move camera over a position on the map
  /// 
  /// The camera will prevent itself from going OOB.
  /// 
  pub fn move_to(&mut self, pos: Pos) {
    
    let mut new_pos = pos.clone();
    new_pos -= (self.screen.x / 2, (self.screen.y) / 2);

    // Boundary checks
    if new_pos.x < 0 { new_pos.x = 0; }
    if new_pos.y < 0 { new_pos.y = 0; }
    if new_pos.x > self.map.x - self.screen.x { new_pos.x = self.map.x - self.screen.x; }
    if new_pos.y > self.map.y - self.screen.y { new_pos.y = self.map.y - self.screen.y; }

    self.pos = -new_pos;

  }

  ///
  /// Return a new `Camera`
  /// 
  /// * `map` - `Pos` that holds the map dimensions
  /// * `screen` - `Pos` that holds the screen dimensions
  /// 
  #[inline]
  pub fn new(map: (isize, isize), screen: (isize, isize)) -> Camera {
    Camera { pos: Pos::origin(), map: Pos::from_tup(map), screen: Pos::from_tup(screen)}
  }

}