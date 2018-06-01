//!
//! Control the game loop, state, and input
//!

use core::tcod::input;

use core::world::World;

// Game state depends on what actions the player can do
use core::object::actions::Actions;

// use core::renderer::Screen;

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
/// Game struct. Holds a player and a floor
/// 
/// * `world` - `World` to represent the game world
/// * `state` - represents the game state
/// 
pub struct Game {
  pub world: World,
  pub state: State
}

impl Game {

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
            _ => { self.world.player.state = Actions::Unknown }
            
          }

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
  pub fn new(map_dim: (isize, isize)) -> Game {

    Game {
      world: World::new(map_dim),
      state: State::New
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

}
