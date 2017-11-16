//! 
//! Start the game
//! 
//! 

extern crate tcod;
// We need tcod::Console to keep our consoles in scope
use tcod::Console;
use tcod::console;
use tcod::input;

use init;
use object::{Pos, Entity};

///
/// Play the game.
/// 
pub fn play() {
  
  // Get root console
  let mut root = init::root();

  let mut exit = false;

  let mut player = Entity {pos: Pos {x: 40, y: 25}, ch: '@'};

  while !(root.window_closed() || exit) {

    root.clear();
    root.put_char(player.pos.x, player.pos.y, player.ch, console::BackgroundFlag::Set);
    root.flush();

    let keypress = root.wait_for_keypress(true);

    match keypress.code {

      input::KeyCode::Escape => exit = true,
      _ => { 
        
        if keypress.printable != ' ' {
          match keypress.printable {

            'h' => player.move_cart(-1, 0),
            'j' => player.move_cart(0, 1),
            'k' => player.move_cart(0, -1),
            'l' => player.move_cart(1, 0),
            'y' => player.move_cart(-1, -1),
            'u' => player.move_cart(1, -1),
            'b' => player.move_cart(-1, 1),
            'n' => player.move_cart(1, 1),
            _ => { println!("{}", keypress.printable) }
            
          }

        } else {
          println!("{:?}", keypress.code);
        }

      }

    }

  }

}