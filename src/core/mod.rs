//! 
//! Hold the `Game` struct and the `play()` function
//! 

// Tcod
extern crate tcod;
use self::tcod::{console, Console};
use core::tcod::input;

// core::world
pub mod world;
use core::world::World;

// core::object
pub mod object;
// Game state depends on what actions the player can do
use core::object::actions::Actions;

// core::renderer
pub mod renderer;
use self::renderer::Renderer;
use self::renderer::RGB;

// core::log
pub mod log;
use self::log::GlobalLog;

// Use init here privately
// After all, what game function requires that something in the initalizer be ran?
mod init;

///
/// Enum representing the state of the game
///
pub enum State {
  // Game just created
  New,
  // Player acted
  Act(Actions),
  // Debug
  Debug
}

///
/// Engine struct. Holds a world, state, and renderer
/// 
pub struct Engine {
  pub world: World,
  pub state: State,
  pub ren: Renderer,
  pub root: console::Root
}

impl Engine {

  ///
  /// Capture keyboard input from tcod and update player state
  /// 
  pub fn process_keypress(&mut self, keypress: input::Key) {

    match keypress.code {
      
      // If the keycode isn't escape we continue checking for important keys
      input::KeyCode::Escape => panic!("Bye"),
      _ => { 
  
        if keypress.printable != ' ' {

          let oldpos = self.world.player.actor.pos.clone();
        
          match keypress.printable {
            
            // Movement keys are bound to vim-like controls (hjklyubn)
            'h' => { 
              self.world.player.actor.move_cart(-1, 0);
              self.world.player.state = Actions::Move;
            },
            'j' => { 
              self.world.player.actor.move_cart(0, 1);
              self.world.player.state = Actions::Move;
            },
            'k' => {
              self.world.player.actor.move_cart(0, -1);
              self.world.player.state = Actions::Move;
            },
            'l' => {
              self.world.player.actor.move_cart(1, 0);
              self.world.player.state = Actions::Move;
            },
            'y' => {
              self.world.player.actor.move_cart(-1, -1);
              self.world.player.state = Actions::Move;
            },
            'u' => {
              self.world.player.actor.move_cart(1, -1);
              self.world.player.state = Actions::Move;
            },
            'b' => {
              self.world.player.actor.move_cart(-1, 1);
              self.world.player.state = Actions::Move;
            },
            'n' => { 
              self.world.player.actor.move_cart(1, 1);
              self.world.player.state = Actions::Move;
            },
            // "Regenerate" current level
            'w' => {
              self.world.test_traverse();
              self.state = State::Debug;
            },
            // Create an empty level for testing
            'q' => {
              self.world.test_empty();
              self.state = State::Debug;
            },
            // Wait
            '.' => { 
              self.world.player.state = Actions::Wait;
            },
            // Go downstars (if possible)
            '>' => { self.world.player.state = Actions::DownStair },
            // Go upstairs (if possible)
            '<' => { self.world.player.state = Actions::UpStair },
            // Unbound key, so we just say we don't know what the player did

            // Debug keypresses

            // Toggle scent
            'r' => {
              self.ren.sc_debug = !self.ren.sc_debug;
              self.ren.draw_all(&mut self.root, &mut self.world);
              self.state = State::Debug;
            },

            // Toggle sound
            't' => {
              self.ren.so_debug = !self.ren.so_debug;
              self.update();
              self.ren.draw_all(&mut self.root, &mut self.world);
              self.state = State::Debug;
            },

            // Toggle FoV
            'f' => {
              self.ren.fov = !self.ren.fov;
              self.update();
              self.ren.draw_all(&mut self.root, &mut self.world);
            },

            _ => { self.world.player.state = Actions::Unknown }
            
          }

          // Make sure player doesn't do anything dumb
          if !self.world.is_valid(self.world.player.actor.pos.x, self.world.player.actor.pos.y) {
            self.world.player.actor.pos = oldpos;
            self.state = State::Act(Actions::Unknown);
            self.world.player.state = Actions::Unknown;
          }

        } 
        
        /* 
        // Prints keycode to console in case if you're trying to find a key that isn't intutive
        else {
          println!("{:?}", keypress.code);
        }
        */

      }

    }
    
  }

  ///
  /// Return a new `Game`
  /// 
  /// This function assumes you will just be passing in tcod::console::Root.width() and height(),
  /// so inputs are i32s instead of usizes (they get converted)
  /// 
  pub fn new() -> Engine {

    // Get map height
    let map_dim = init::map_dimensions();

    let root = init::root();

    Engine {
      world: World::new(map_dim),
      state: State::New,
      ren: Renderer::new(
        map_dim, 
        (root.width() as isize, root.height() as isize), 
        init::console_height(),
        init::panel_width()
      ),
      // Get root console
      root: root,

    }
    
  }

  ///
  /// Update the game state, then update depending on the new state
  ///
  pub fn update(&mut self) {

    match self.state {
      State::Debug => (),
      _ => self.state = State::Act(self.world.player.state.clone())
    }

    match &self.state {
      // Moving or waiting prompts a world update
      &State::Act(Actions::Move) | &State::Act(Actions::Wait) => self.world.update(),

      // Trying to go up and downstairs prompts the respective response from world
      &State::Act(Actions::DownStair) => {
        if self.world.can_go_down() {
          self.world.go_down();
        }
      },
      &State::Act(Actions::UpStair) => {
        if self.world.can_go_up() {
          self.world.go_up();
        }
      }
      
      // All other variants are dropped
      _ => ()
    }
    
  }

  ///
  /// Play the game.
  /// 
  pub fn play(&mut self) {

    // Some starting messages, will be removed in later versions
    let mut log = GlobalLog.lock().unwrap();
    log.push(("Welcome to Edgequest", RGB(255, 0, 255)));
    log.push(("Move with vim keys", RGB(255, 255, 255)));
    log.push(("esc to quit, w to regenerate", RGB(255, 255, 255)));
    log.push(("r to toggle scent, t to toggle sound", RGB(255, 255, 255)));
    log.push(("f to toggle FoV", RGB(255, 255, 255)));
    drop(log);

    // Initial update
    self.update();

    // Draw all and capture keypresses
    while !self.root.window_closed() {

      // Draw what the camera sees
      self.ren.draw_all(&mut self.root, &mut self.world);
      
      // Capture game keys (Keys that change the state of the player)
      // This is what gives it the turn based nature, i.e. waits for player input before
      // doing anything
      let keypress = self.root.wait_for_keypress(true);
      self.process_keypress(keypress);

      // Update game
      self.update();

    } 

  }

}