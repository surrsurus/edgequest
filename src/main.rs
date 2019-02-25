//! 
//! Edgequest Season 2
//! 
//! Edgequest is a roguelike that probably won't ever be finished due to the scope
//! of things I want to be in the game, but so far it's a pretty great tech demo of
//! interesting modern roguelike mechanics.
//! 
//! The overarching design philosophy of edgequest is to treat the smallest 'atomic' elements as
//! state machines, where the phrase 'atomic' is simply refering to the fact they cannot be broken down any smaller
//! than they currently are. These state machines can then interact by the interfaces that own them, and be
//! processed into complex events and patterns.
//! 
//! While this does make things more straightforward conceptually, the implementation is very non-intuitive.
//! Creatures and tiles are currently the smallest atomic objects with state (though creature is made of several component parts).
//! Creatures manipulate their state via their AI and the world struct handles their interactions with other atomic elements and the
//! various other stimuli present. This means that the world really a high-level construct, rather than the very base that one would assume
//! creatures to interact with. In short, the world owns the creatures, and the creatures own their state.
//! 
//! The player is also a creature, but their state is modified and maintained at the highest level possible at the engine to
//! process key events through tcod, but can still be accessed via the world.
//! 
//! Ultimately, this process is very much a top-down approach, and this has it's advantages as it
//! allows us to avoid a lot of ownership issues traditional OO causes, as objects are manipulated from top-down,
//! but also introduces the strange way of doing things currently.
//! 
//! Edgequest does not use a traditional ECS for managing entities and their components, a pseudo ECS arises from
//! from the rust type system and it's powerful match syntax. Entities have states and properties which are both enums, meaning that
//! the world can simply match these enums to functionality. Properties can be added and removed from creatures and tiles easily and on the fly,
//! and adding new ones is also trivial provided the relevant matches are updated.
//! 

// Clippy config
#![allow(clippy::needless_return)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::single_match)]

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