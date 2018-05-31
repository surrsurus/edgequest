//!
//! Control the game loop, state, and input
//!

use core::tcod::input;
use core::tcod::map::FovAlgorithm;

use core::log::GlobalLog;

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
  /// Capture keyboard input from tcod
  /// 
  pub fn process_keypress(&mut self, keypress: input::Key) {

    match keypress.code {
      
      // If the keycode isn't escape we continue checking for important keys
      input::KeyCode::Escape => panic!("Bye"),
      _ => { 
  
        if keypress.printable != ' ' {

          let oldpos = self.world.player.fighter.pos.clone();
        
          match keypress.printable {
            
            // Movement keys are bound to vim-like controls (hjklyubn)
            'h' => { 
              self.world.player.fighter.move_cart(-1, 0);
              self.state = State::Act(Actions::Move);
              self.world.player.state = Actions::Move;
            },
            'j' => { 
              self.world.player.fighter.move_cart(0, 1);
              self.state = State::Act(Actions::Move);
              self.world.player.state = Actions::Move;
            },
            'k' => {
              self.world.player.fighter.move_cart(0, -1);
              self.state = State::Act(Actions::Move);
              self.world.player.state = Actions::Move;
            },
            'l' => {
              self.world.player.fighter.move_cart(1, 0);
              self.state = State::Act(Actions::Move);  
              self.world.player.state = Actions::Move;
            },
            'y' => {
              self.world.player.fighter.move_cart(-1, -1);
              self.state = State::Act(Actions::Move);
              self.world.player.state = Actions::Move;
            },
            'u' => {
              self.world.player.fighter.move_cart(1, -1);
              self.state = State::Act(Actions::Move);
              self.world.player.state = Actions::Move;
            },
            'b' => {
              self.world.player.fighter.move_cart(-1, 1);
              self.state = State::Act(Actions::Move);
              self.world.player.state = Actions::Move;
            },
            'n' => { 
              self.world.player.fighter.move_cart(1, 1);
              self.state = State::Act(Actions::Move);
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
              self.state = State::Act(Actions::Wait);
              self.world.player.state = Actions::Wait;
            },
            // Go downstars (if possible)
            '>' => { self.state = State::Act(Actions::DownStair) },
            // Go upstairs (if possible)
            '<' => { self.state = State::Act(Actions::UpStair) },
            // Unbound key, so we just say we don't know what the player did
            _ => { self.state = State::Act(Actions::Unknown) }
            
          }

          if !self.world.is_valid(self.world.player.fighter.pos.x, self.world.player.fighter.pos.y) {
            self.world.player.fighter.pos = oldpos;
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
  /// Update the game depending on the state
  ///
  pub fn update(&mut self) {
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
    // TODO ...Does this need to be here? :thinking:
    self.world.tcod_map.compute_fov(self.world.player.fighter.pos.x as i32, self.world.player.fighter.pos.y as i32, 20, true, FovAlgorithm::Shadow);
  }

}
