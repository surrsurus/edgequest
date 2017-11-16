//! 
//! Hold the `Game` struct and the `play()` function
//! 

extern crate tcod;

use tcod::Console;
use tcod::console::Root;
use tcod::colors::Color;
use tcod::input;

use dungeon::Dungeon;

use init;

use object::{Pos, Entity, Map, Tile};

///
/// Game struct. Holds a player and a map
/// 
/// * `player` - `Entity` to represent the player
/// * `map` - `Map` object to represent the current floor
/// 
pub struct Game {
    pub player: Entity,
    pub map: Map,
    pub dungeon: Dungeon,
}

impl Game {

  ///
  /// Return a new player `Entity`
  /// 
  pub fn new_player() -> Entity {
    return Entity::new(
      Pos::new(40, 25), 
      '@', 
      Color::new(255, 255, 255), 
      Color::new(0, 0, 0)
    );
  }

  ///
  /// Return a new empty `Map`
  /// 
  /// This function assumes you will just be passing in tcod::console::Root.width() and height(),
  /// so inputs are i32s instead of usizes (they get converted)
  /// 
  pub fn new_map(width: i32, height: i32) -> Map {
    return Map::new(
      width as usize, 
      height as usize, 
      Vec::<Tile>::new(), 
      Vec::<Entity>::new()
    );
  }

  pub fn new_dungeon(width: i32, height: i32) -> Dungeon {
    return Dungeon::new(width, height, 15);
  }

  ///
  /// Return a new `Game`
  /// 
  /// This function assumes you will just be passing in tcod::console::Root.width() and height(),
  /// so inputs are i32s instead of usizes (they get converted)
  /// 
  pub fn new(width: i32, height: i32) -> Game {

    let mut g = Game {player: Game::new_player(), map: Game::new_map(width, height), dungeon: Game::new_dungeon(width, height) };

    for x in 0..g.dungeon.w {
      for y in 0..g.dungeon.h {
        if g.dungeon.grid[x as usize][y as usize] == 1 {
          g.map.tile_vec.push(
            Tile::new(
                Pos::new(x, y), 
                ' ', 
                Color::new(255, 255, 255), 
                Color::new(0, 0, 0), 
                false
              )
            );
        } else {
          g.map.tile_vec.push(
            Tile::new(
              Pos::new(x, y), 
              ' ', 
              Color::new(255, 255, 255), 
              Color::new(33, 33, 33), 
              true
            )
          );
        }
        
      }

    }

    g.player.pos = g.dungeon.get_starting_location();

    return g;
  }

  ///
  /// Draw all.
  /// 
  /// You'll have to render this console to root (unless you passed root in)
  /// and always `flush()` the root console.
  /// 
  pub fn draw_all(&mut self, con: &mut tcod::console::Console) {
    
    // Clear console
    con.clear();

    // Draw tiles
    for t in self.map.tile_vec.iter() {
      if t.entity.glyph == ' ' {
        con.set_char_background(
          t.entity.pos.x,
          t.entity.pos.y,
          t.entity.bg,
          tcod::console::BackgroundFlag::Set
        );
      } else {
        con.put_char_ex(
          t.entity.pos.x, 
          t.entity.pos.y, 
          t.entity.glyph,
          t.entity.fg,
          t.entity.bg
        );
      }

    }

    // Draw entities
    for e in self.map.entity_vec.iter() {
        con.put_char_ex(e.pos.x, e.pos.y, e.glyph, e.fg, e.bg);
    }

    // Draw player
    con.put_char_ex(
      self.player.pos.x, 
      self.player.pos.y, 
      self.player.glyph, 
      self.player.fg, 
      self.player.bg
    );

  }

  ///
  /// Capture keyboard input and process it with tcod
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

          for t in self.map.tile_vec.iter() {
            if t.blocks && t.entity.pos == self.player.pos {
              self.player.pos = oldpos;
            }
          }

        } else {
          println!("{:?}", keypress.code);
        }

      }

    }
  }

}

///
/// Play the game.
/// 
pub fn play() {
  
  // Get root console
  let mut root = init::root();

  // Get a new game
  let mut game = Game::new(root.width(), root.height());

  // Draw all and capture keypresses
  while !(root.window_closed()) {

    game.draw_all(&mut root);

    root.flush();

    game.capture_keypress(&mut root);

  }

}