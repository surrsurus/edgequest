//! 
//! Hold the `Game` struct and the `play()` function
//! 

// Tcod
extern crate tcod;
use self::tcod::Console;

// core::dungeon
pub mod dungeon;

// core::init
pub mod init;

// core::object
pub mod object;
use self::object::Pos;

pub mod renderer;
use self::renderer::Renderer;

pub mod game;
use self::game::Game;

///
/// Play the game.
/// 
pub fn play() {
  
  // Get root console
  let mut root = init::root();

  // Get map height
  let map_dim = init::map_dimensions();

  // Get a new camera
  let mut cam = Renderer::new(map_dim, Pos::new(root.width(), root.height()));

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