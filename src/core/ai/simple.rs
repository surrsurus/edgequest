use core::ai::AI;

use core::dungeon::map::Grid;
use core::dungeon::map::Tile;

use core::object::Fighter;

extern crate rand;
use self::rand::{thread_rng, Rng};

pub struct SimpleAI;

impl SimpleAI {
  pub fn new() -> SimpleAI {
    SimpleAI {}
  }
}

impl AI for SimpleAI {
  
  fn take_turn(&mut self, map: &Grid<Tile>, _player: &Fighter, me: &mut Fighter) {

    let mut rng = thread_rng();
    let mut dice : i32;
    
    let mut x : usize;
    let mut y : usize;
    loop {
      x = me.pos.x as usize;
      y = me.pos.y as usize;
      dice = rng.gen_range(1, 5);
      match dice {
        1 => x += 1,
        2 => x -= 1,
        3 => y += 1,
        4 => y -= 1,
        _ => unreachable!("dice machine broke")
      }

      if !map[x][y].blocks { break; }
    }
    
    me.pos.x = x as isize;
    me.pos.y = y as isize;

  }

}