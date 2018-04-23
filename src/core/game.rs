
use core::world::World;

use core::tcod::input;

// use core::renderer::Screen;

///
/// Game struct. Holds a player and a floor
/// 
/// * `player` - `Entity` to represent the player
/// * `floor` - `Floor` object to represent the current floor the player is on
/// 
pub struct Game {
  pub world: World,
  pub state: String
}

impl Game {

  ///
  /// Capture keyboard input from tcod
  /// 
  pub fn process_keypress(&mut self, keypress: input::Key) {

    match keypress.code {

      input::KeyCode::Escape => panic!("Bye"),
      _ => { 
  
        if keypress.printable != ' ' {

          let oldpos = self.world.player.pos.clone();
        
          match keypress.printable {

            'h' => { 
              self.world.player.move_cart(-1, 0);
              self.state = "act".to_string();
            },
            'j' => { 
              self.world.player.move_cart(0, 1);
              self.state = "act".to_string();
            },
            'k' => {
              self.world.player.move_cart(0, -1);
              self.state = "act".to_string();
            },
            'l' => {
              self.world.player.move_cart(1, 0);
              self.state = "act".to_string();  
            },
            'y' => {
              self.world.player.move_cart(-1, -1);
              self.state = "act".to_string();
            },
            'u' => {
              self.world.player.move_cart(1, -1);
              self.state = "act".to_string();
            },
            'b' => {
              self.world.player.move_cart(-1, 1);
              self.state = "act".to_string();
            },
            'n' => { 
              self.world.player.move_cart(1, 1);
              self.state = "act".to_string();
            },
            '.' => { self.state = "act".to_string() },
            _ => { self.state = "unknown".to_string() }
            
          }

          if !self.world.cur_dungeon.is_valid(self.world.player.pos.x as usize, self.world.player.pos.y as usize) {
            self.world.player.pos = oldpos;
            self.state = "unknown".to_string();
          }

        } else {
          println!("{:?}", keypress.code);
        }

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
      state: "new".to_string()
    }
    
  }

}
