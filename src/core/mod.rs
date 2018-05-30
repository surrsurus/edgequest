//! 
//! Hold the `Game` struct and the `play()` function
//! 

// Tcod
extern crate tcod;
use self::tcod::Console;

// core::world
pub mod world;

// core::object
pub mod object;

// core::renderer
pub mod renderer;
use self::renderer::Renderer;

// core::log
pub mod log;
use self::log::GlobalLog;

// Use game privately
// Game is pretty much the most top level thing besides core
mod game;
use self::game::Game;

// Use init here privately
// After all, what game function requires that something in the initalizer be ran?
mod init;

///
/// Play the game.
/// 
pub fn play() {
  
  // Get root console
  let mut root = init::root();

  // Get map height
  let map_dim = init::map_dimensions();

  // Get a new renderer
  let mut ren = Renderer::new(map_dim, (root.width() as isize, root.height() as isize));

  // Get a new game
  let mut game = Game::new(map_dim);
  game.update();

  // Draw all and capture keypresses
  while !root.window_closed() {

    // Draw what the camera sees
    ren.draw_all(&mut root, &mut game.world);
    
    // Capture keypresses
    let keypress = root.wait_for_keypress(true);
    // Capture debug keys
    if keypress.printable == 'r' {
      ren.sc_debug = !ren.sc_debug;
      ren.draw_all(&mut root, &mut game.world);
    }
    if keypress.printable == 't' {
      ren.so_debug = !ren.so_debug;
      game.update();
      ren.draw_all(&mut root, &mut game.world);
    }
    if keypress.printable == 'f' {
      ren.fov = !ren.fov;
      game.update();
      ren.draw_all(&mut root, &mut game.world);
    }
    // Capture game keys (Keys that change the state of the player)
    game.process_keypress(keypress);

    // Update game
    game.update();

  } 

}