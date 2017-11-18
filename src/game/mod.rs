//! 
//! Hold the `Game` struct and the `play()` function
//! 

// Tcod
extern crate tcod;
use self::tcod::Console;
use self::tcod::console::Root;
use self::tcod::input;

// game::dungeon
pub mod dungeon;
use self::dungeon::Dungeon;

// game::init
pub mod init;

// game::object
pub mod object;
use self::object::{Pos, Entity, Floor, Tile};

///
/// Helper for rendering things to the screen
/// 
/// Tracks the player and automatically scrolls the screen around to match where they go.
/// This will never try to draw things outside of the given dimensions due to the way it handles
/// determining whether something should be drawn or not. 
/// 
pub struct Camera {

  // Position that the camera is panned to on the map
  // Must be within map bounds, or camera will just go to the region,
  // though the target won't be exactly in the center of the screen.
  camera: Pos,

  // Map dimensions
  map: Pos,

  // Screen dimensions
  screen: Pos,

}

impl Camera {

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
        tcod::console::BackgroundFlag::Set
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
  /// Return a new `Camera`
  /// 
  /// * `map` - `Pos` that holds the map dimensions
  /// * `screen` - `Pos` that holds the screen dimensions
  /// 
  pub fn new(map: Pos, screen: Pos) -> Camera {
    return Camera { camera: Pos::origin(), map: map, screen: screen };
  }

}

///
/// Game struct. Holds a player and a floor
/// 
/// * `player` - `Entity` to represent the player
/// * `floor` - `Floor` object to represent the current floor the player is on
/// 
pub struct Game {

  pub player: Entity,
  pub floor: Floor,

  dungeon: Dungeon,

}

impl Game {

  ///
  /// Capture keyboard input from tcod
  /// 
  pub fn capture_keypress(&mut self, root: &mut Root) {

    let keypress = root.wait_for_keypress(true);

    match keypress.code {

      input::KeyCode::Escape => panic!("Bye"),
      _ => { 
  
        if keypress.printable != ' ' {

          let oldpos = self.player.pos.clone();
        
          match keypress.printable {

            'h' => self.player.move_cart(-1, 0),
            'j' => self.player.move_cart(0, 1),
            'k' => self.player.move_cart(0, -1),
            'l' => self.player.move_cart(1, 0),
            'y' => self.player.move_cart(-1, -1),
            'u' => self.player.move_cart(1, -1),
            'b' => self.player.move_cart(-1, 1),
            'n' => self.player.move_cart(1, 1),
            _ => { println!("{}", keypress.printable) }
            
          }

          for t in self.floor.tile_vec.iter() {
            if t.blocks && t.entity.pos == self.player.pos {
              self.player.pos = oldpos;
            }
          }

        } else {
          println!("{:?}", keypress.code);
        }

      }

    }
    
  }

  ///
  /// Get a new `Dungeon`
  /// 
  pub fn new_dungeon(map_dim: Pos) -> Dungeon {
    return Dungeon::new(map_dim.x, map_dim.y, (map_dim.x + map_dim.y) / 10);
  }

  ///
  /// Return a new empty `Floor`
  /// 
  /// This function assumes you will just be passing in tcod::console::Root.width() and height(),
  /// so inputs are i32s instead of usizes (they get converted)
  /// 
  pub fn new_floor(map_dim: Pos) -> Floor {
    return Floor::new(
      map_dim.x as usize, 
      map_dim.y as usize, 
      Vec::<Tile>::new(), 
      Vec::<Entity>::new()
    );
  }

  ///
  /// Return a new player `Entity`
  /// 
  #[inline]
  pub fn new_player() -> Entity {
    return Entity::new(
      Pos::new(40, 25), 
      '@', 
      (255, 255, 255), 
      (0, 0, 0)
    );
  }

  ///
  /// Return a new `Game`
  /// 
  /// This function assumes you will just be passing in tcod::console::Root.width() and height(),
  /// so inputs are i32s instead of usizes (they get converted)
  /// 
  pub fn new(map_dim: Pos) -> Game {

    let mut g = Game {
      player: Game::new_player(), 
      floor: Game::new_floor(map_dim), 
      dungeon: Game::new_dungeon(map_dim) 
    };

    for x in 0..g.dungeon.w {
      for y in 0..g.dungeon.h {
        if g.dungeon.grid[x as usize][y as usize] == 1 {
          g.floor.tile_vec.push(
            Tile::new(
              Pos::new(x, y), 
              ' ', 
              (255, 255, 255), 
              (0, 0, 0), 
              false
            )
          );
        } else {
          g.floor.tile_vec.push(
            Tile::new(
              Pos::new(x, y), 
              ' ', 
              (255, 255, 255), 
              (33, 33, 33), 
              true
            )
          );
        }
        
      }

    }
    
    let start_tup = g.dungeon.get_starting_location();
    g.player.pos.x = start_tup.0;
    g.player.pos.y = start_tup.1;

    return g;
    
  }

}

///
/// Play the game.
/// 
pub fn play() {
  
  // Get root console
  let mut root = init::root();

  // Get map height
  let map_dim = init::map_dimensions();

  // Get a new camera
  let mut cam = Camera::new(map_dim, Pos::new(root.width(), root.height()));

  // Get a new game
  let mut game = Game::new(map_dim);

  // Draw all and capture keypresses
  while !(root.window_closed()) {

    // AI actions go here

    // Draw what the camera sees
    cam.draw_all(&mut root, &game);

    // Flush all draws to root
    root.flush();

    // Capture keypresses
    game.capture_keypress(&mut root);

  }

}