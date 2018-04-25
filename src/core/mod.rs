//! 
//! Hold the `Game` struct and the `play()` function
//! 

// Tcod
extern crate tcod;
use self::tcod::Console;

// core::world
pub mod world;

// core::init
pub mod init;

// core::object
pub mod object;

// core::renderer
pub mod renderer;
use self::renderer::Renderer;

// core::game
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

  // Get a new renderer
  let mut ren = Renderer::new(map_dim, (root.width() as isize, root.height() as isize));

  // Get a new game
  let mut game = Game::new(map_dim);
  game.update();

  // Draw all and capture keypresses
  while !root.window_closed() {

    // Draw what the camera sees
    ren.draw_all(&mut root, &mut game.world);

    // Flush all draws to root
    root.flush();

    // Capture keypresses
    let keypress = root.wait_for_keypress(true);
    if keypress.printable == 'r' {
      ren.sc_debug = !ren.sc_debug;
      ren.draw_all(&mut root, &mut game.world);
    }
    if keypress.printable == 'f' {
      ren.fov = !ren.fov;
      game.update();
      ren.draw_all(&mut root, &mut game.world);
    }
    game.process_keypress(keypress);

    // Update game
    game.update();

  } 

}