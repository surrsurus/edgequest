use core::world::dungeon::map::Grid;
use core::world::dungeon::map::{Tile, TileType};

use core::object::{Actions, Creature, Fighter};
use core::object::ai::AI;
use core::object::ai::{MovementTypes};

extern crate rand;
use self::rand::{thread_rng, Rng};

///
/// Composite AI
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompositeAI {
  movement: MovementTypes
}

impl CompositeAI {
  pub fn new(mt: MovementTypes) -> CompositeAI {
    CompositeAI {
      movement: mt
    }
  }

  pub fn blink(&mut self, map: &Grid<Tile>, _player: &Creature, me: &mut Fighter) -> (isize, isize) {
    let mut rng = thread_rng();
    let mut x = me.pos.x;
    let mut y = me.pos.y;
    x += rng.gen_range(-8, 8);
    y += rng.gen_range(-8, 8);
    if x < 0 {
      x = 0;
    }
    if y < 0 {
      y = 0;
    }
    if y >= (map[0].len() - 1) as isize {
      y = (map[0].len() - 1) as isize;
    }
    if x >= (map.len() - 1) as isize {
      x = (map.len() - 1) as isize;
    }
    return (x, y);
  }

  pub fn movement_blink(&mut self, map: &Grid<Tile>, _player: &Creature, me: &mut Fighter) -> (isize, isize, Actions) {
    let mut x = me.pos.x;
    let mut y = me.pos.y;
    let mut rng = thread_rng();
    let dice : i32;
    let mut state : Actions;
    dice = rng.gen_range(1, 6);
    state = Actions::Move;
    match dice {
      1 => x += 1,
      2 => x -= 1,
      3 => y += 1,
      4 => y -= 1,
      5 => {
        let bpos = self.blink(map, _player, me);
        x = bpos.0;
        y = bpos.1;
        state = Actions::Blink;
      },
      _ => unreachable!("dice machine broke")
    }
    return (x, y, state);
  }

  pub fn movement_dumb(&mut self, _map: &Grid<Tile>, _player: &Creature, me: &mut Fighter) -> (isize, isize, Actions) {
    let mut x = me.pos.x;
    let mut y = me.pos.y;
    let mut rng = thread_rng();
    let dice : i32;
    dice = rng.gen_range(1, 5);
    match dice {
      1 => x += 1,
      2 => x -= 1,
      3 => y += 1,
      4 => y -= 1,
      _ => unreachable!("dice machine broke")
    }
    return (x, y, Actions::Move);
  }

  pub fn movement_track(&mut self, map: &Grid<Tile>, player: &Creature, me: &mut Fighter) -> (isize, isize, Actions) {
    let mut x = me.pos.x;
    let mut y = me.pos.y;
    let mut state = Actions::Wait;

    let distance = me.pos ^ player.fighter.pos;

    if distance < 20.0 && distance > 2.00 {

      // Move x
      if x < player.fighter.pos.x {
        x += 1;
        state = Actions::Move;
      } else if x > player.fighter.pos.x {
        x -= 1;
        state = Actions::Move;
      }

      // Check
      match map[x as usize][y as usize].tiletype {
        TileType::Wall => x = me.pos.x,
        _ => {}
      }

      // Move y
      if y < player.fighter.pos.y {
        y += 1;
        state = Actions::Move;
      } else if y > player.fighter.pos.y {
        y -= 1;
        state = Actions::Move;
      }

      // Check
      match map[x as usize][y as usize].tiletype {
        TileType::Wall => y = me.pos.y,
        _ => {}
      }

    }
    return (x, y, state);
  }

}

impl AI for CompositeAI {
  
  ///
  /// Walk around randomly
  ///
  fn take_turn(&mut self, map: &Grid<Tile>, player: &Creature, me: &mut Fighter) -> Actions {
    
    let mut mpos : (isize, isize, Actions);
    let mut x : isize;
    let mut y : isize;
    let mut count : usize = 0;
    let mut state : Actions;
    loop {
      count += 1;
      match self.movement {
        MovementTypes::Blink => {
          mpos = self.movement_blink(map, player, me);
          x = mpos.0;
          y = mpos.1;
          state = mpos.2;
        },
        MovementTypes::Track => {
          mpos = self.movement_track(map, player, me);
          x = mpos.0;
          y = mpos.1;
          state = mpos.2;
        },
        MovementTypes::Dumb => {
          mpos = self.movement_dumb(map, player, me);
          x = mpos.0;
          y = mpos.1;
          state = mpos.2;
        }
        _ => unreachable!("All movement types should be individually matched")
      }      

      match map[x as usize][y as usize].tiletype {
        TileType::Floor => break,
        _ => {
          if count > 100 {
            x = me.pos.x;
            y = me.pos.y;
            state = Actions::Wait;
            break; 
          }
        }
      }
    }
    
    me.pos.x = x as isize;
    me.pos.y = y as isize;

    return state;

  }

}