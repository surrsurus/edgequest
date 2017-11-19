use core::object::{Pos, Entity, Floor, Tile, RGB};
use core::dungeon::Dungeon;
use core::tcod::console::Root;
use core::tcod::input;

///
/// Game struct. Holds a player and a floor
/// 
/// * `player` - `Entity` to represent the player
/// * `floor` - `Floor` object to represent the current floor the player is on
/// 
pub struct Game {

  pub player: Entity,
  pub floor: Floor,

  dungeon: Dungeon,

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

          for t in self.floor.tile_vec.iter() {
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

  ///
  /// Get a new `Dungeon`
  /// 
  pub fn new_dungeon(map_dim: Pos) -> Dungeon {
    return Dungeon::new(map_dim.x, map_dim.y, (map_dim.x + map_dim.y) / 10);
  }

  ///
  /// Return a new empty `Floor`
  /// 
  /// This function assumes you will just be passing in tcod::console::Root.width() and height(),
  /// so inputs are i32s instead of usizes (they get converted)
  /// 
  pub fn new_floor(map_dim: Pos) -> Floor {
    return Floor::new(
      map_dim.x as usize, 
      map_dim.y as usize, 
      Vec::<Tile>::new(), 
      Vec::<Entity>::new()
    );
  }

  ///
  /// Return a new player `Entity`
  /// 
  #[inline]
  pub fn new_player() -> Entity {
    return Entity::new(
      Pos::new(40, 25), 
      '@', 
      RGB(255, 255, 255), 
      RGB(0, 0, 0)
    );
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
      floor: Game::new_floor(map_dim), 
      dungeon: Game::new_dungeon(map_dim) 
    };

    for x in 0..g.dungeon.w {
      for y in 0..g.dungeon.h {
        if g.dungeon.grid[x as usize][y as usize] == 1 {
          g.floor.tile_vec.push(
            Tile::new(
              Pos::new(x, y), 
              ' ', 
              RGB(255, 255, 255), 
              RGB(0, 0, 0), 
              false
            )
          );
        } else {
          g.floor.tile_vec.push(
            Tile::new(
              Pos::new(x, y), 
              ' ', 
              RGB(255, 255, 255), 
              RGB(33, 33, 33), 
              true
            )
          );
        }
        
      }

    }
    
    let start_tup = g.dungeon.get_starting_location();
    g.player.pos.x = start_tup.0;
    g.player.pos.y = start_tup.1;

    return g;
    
  }

}
