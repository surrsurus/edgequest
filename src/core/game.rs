use core::dungeon::Dungeon;

use core::dungeon::map::Grid;

use core::object::{Pos, Fighter, RGB};

use core::tcod::console::Root;
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
  // pub screen: Box<Screen>,

  pub dungeon: Dungeon,
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

          if self.dungeon.grid.0[self.player.pos.x as usize][self.player.pos.y as usize].blocks {
            self.player.pos = oldpos;
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
    Dungeon::new(map_dim.x as usize, map_dim.y as usize)
  }

  ///
  /// Return a new player `Entity`
  /// 
  #[inline]
  pub fn new_player() -> Fighter {
    Fighter::new(
      "Player".to_string(),
      '@', 
      Pos::new(40, 25), 
      RGB(255, 255, 255), 
      RGB(0, 0, 0)
    )
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
      dungeon: Game::new_dungeon(map_dim) 
    };

    g.dungeon.build();
    
    g.player.pos = g.dungeon.get_starting_location();

    return g;
    
  }

}
