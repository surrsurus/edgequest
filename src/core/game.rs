use core::dungeon::Dungeon;

use core::object::Fighter;

use core::tcod::input;

// use core::renderer::Screen;

///
/// Game struct. Holds a player and a floor
/// 
/// * `player` - `Entity` to represent the player
/// * `floor` - `Floor` object to represent the current floor the player is on
/// 
pub struct Game {
  pub player: Fighter,

  pub dungeon: Dungeon,
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

          let oldpos = self.player.pos.clone();
        
          match keypress.printable {

            'h' => { 
              self.player.move_cart(-1, 0);
              self.state = "act".to_string();
            },
            'j' => { 
              self.player.move_cart(0, 1);
              self.state = "act".to_string();
            },
            'k' => {
              self.player.move_cart(0, -1);
              self.state = "act".to_string();
            },
            'l' => {
              self.player.move_cart(1, 0);
              self.state = "act".to_string();  
            },
            'y' => {
              self.player.move_cart(-1, -1);
              self.state = "act".to_string();
            },
            'u' => {
              self.player.move_cart(1, -1);
              self.state = "act".to_string();
            },
            'b' => {
              self.player.move_cart(-1, 1);
              self.state = "act".to_string();
            },
            'n' => { 
              self.player.move_cart(1, 1);
              self.state = "act".to_string();
            },
            '.' => { self.state = "act".to_string() },
            _ => { self.state = "unknown".to_string() }
            
          }

          if !self.dungeon.is_valid(self.player.pos.x as usize, self.player.pos.y as usize) {
            self.player.pos = oldpos;
            self.state = "unknown".to_string();
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
  pub fn new_dungeon(map_dim: (isize, isize)) -> Dungeon {
    Dungeon::new((map_dim.0 as usize, map_dim.1 as usize))
  }

  ///
  /// Return a new player `Entity`
  /// 
  #[inline]
  pub fn new_player() -> Fighter {
    Fighter::new(
      "Player".to_string(),
      '@', 
      (40, 25), 
      (255, 255, 255), 
      (0, 0, 0)
    )
  }

  ///
  /// Return a new `Game`
  /// 
  /// This function assumes you will just be passing in tcod::console::Root.width() and height(),
  /// so inputs are i32s instead of usizes (they get converted)
  /// 
  pub fn new(map_dim: (isize, isize)) -> Game {

    let mut g = Game {
      player: Game::new_player(), 
      dungeon: Game::new_dungeon(map_dim),
      state: "new".to_string()
    };

    g.dungeon.build();
    
    let start_loc = g.dungeon.get_starting_location();
    g.player.pos.x = start_loc.0 as isize;
    g.player.pos.y = start_loc.1 as isize;

    return g;
    
  }

  ///
  /// Update the game world
  /// 
  pub fn update_world(&mut self) {
    if self.state == "act".to_string() {
      self.dungeon.update_scent(self.player.pos.as_tup());
    }
  }

}
