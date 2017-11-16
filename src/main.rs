//! Codename for the Edgequest Revival Project

extern crate tcod;

// #[macro_use] extern crate serde_derive;
// extern crate serde_yaml;

// Local imports. Set as public so docs are generated for them
pub mod init;
pub mod config;
pub mod game;
pub mod object;
pub mod tile;
pub mod map;

fn main() {
  game::play();
}