//! 
//! Edgequest Season 2
//! 

extern crate tcod;
extern crate rand;

// Local imports. Set as public so docs are generated for them
pub mod dungeon;
pub mod game;
pub mod init;
pub mod object;

// Defer to game to start playing.
fn main() {
  game::play();
}