//! 
//! Edgequest Season 2
//! 

// Local imports. Set as public so docs are generated for them
pub mod game;

// Defer to game to start playing.
fn main() {
  game::play();
}