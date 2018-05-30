//! 
//! Edgequest Season 2
//! 

// Local imports. Set as public so docs are generated for them
pub mod core;

// For our log
#[macro_use]
extern crate lazy_static;

// Defer to game to start playing.
fn main() {
  core::play();
}