//! 
//! Edgequest Season 2
//! 
//! Edgequest is a roguelike that probably won't ever be finished due to the scope
//! of things I want to be in the game, but so far it's a pretty great tech demo of
//! interesting modern roguelike mechanics.
//! 
//! Edgequest leverages rust's fantastic type system to create game systems that are extendable,
//! modifiable, and (relatively) straightforward while remaining safe and fast. A lot of the core logic
//! uses the haskell-esque pattern matching to drive the descision making.
//! 

// Local imports for all game files
//
// We set as public so docs are generated for them
pub mod core;

// For our log
//
// From the GitHub: `Using this macro, it is possible to have statics that 
// require code to be executed at runtime in order to be initialized. 
// This includes anything requiring heap allocations, like vectors or hash maps, 
// as well as anything that requires non-const function calls to be computed.
// 
// Allows us to have `Mutex::new(Log::new());` as static reference, meaning multiple
// portions of the code can access the reference to the log via locking the mutex,
// writing to the log with it's impls, and then freeing the mutex so another piece of code
// can lock it down. 
//
// Seems to be pretty dependent on the fact that we only have one thread
// that runs concurrently so we don't accidentally try to get the mutex twice at once and
// miserably fail writing to the log, but I'm not 100% sure about that.
#[macro_use]
extern crate lazy_static;

// For our config loading
//
// Serde allos us to serialize files such as YAML directly into rust structs, meaning
// we put virtually no effort into writing the code to load such files
#[macro_use]
extern crate serde_derive;

// Defer to game to start playing.
fn main() {
  core::Engine::new().play();
}