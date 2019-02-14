//!
//! Global, mutable log.
//!
//! You may think, "Global and mutable?" and wonder how the borrow checker doesn't completely have a meltdown.
//!
//! Well I do too.
//!

// What can I say, I think `GlobalLog` is prettier than GLOBALLLOG
#![allow(non_upper_case_globals)]

///
/// How to use it
///
/// Import log and use the macro `log!()` and pass in the string/RGB tuple. Anything else and im pretty sure it panics
///

///
/// How it actually works
///
/// So here's the thought process of this whole thing. I realized that I need some way for various detached objects to have some
/// way to communicate what they are doing directly to the player without creating a mess of spaghetti. The best idea I came up with
/// was to take some sort of global singleton that can be mutated and then read from the renderer to be drawn to the screen.
///
/// So here's the breakdown.
///
/// A Mutex is a "mutual exclusion primitive useful for protecting shared data", which is essentially just an RAII construct
/// that guarantees that the resource is available to any function that may access the object. In order to access the static refernece,
/// we must `lock` the mutex, which simply blocks the current thread until the mutex is able to be acquired.
/// Since we are single-threaded, this is a non-issue in terms of runtime.
///
/// This mutex then provides a static, mutable reference to the log which then can have it's methods called. After the log is done being used,
/// the reference to the log must be dropped. This does not remove the static referece, but merely allows the mutex to be freed and thus
/// used later by another resource.
///
/// Note that rust mutexes can be poisoned. Essentially, if I lock the mutex then panic a thread, that mutex is no longer considered safe and
/// thus poisoned, which is why it must be unwrapped. Since this is single-threaded, if the thread panics the game doesn't function meaning this
/// in theory, is not an issue.
///
/// In order to however *even fundamentally expose a mutex to the rest of the program via a static reference* we need the lazy_static
/// macro which is the final key to getting it all working. And for fluff, non_uppercase_globals because.
///
/// Then it's slapped into a macro.
///
use std::sync::Mutex;

use core::renderer::RGB;

///
/// A log just wraps some strings with a color value to be printed and look pretty
///
pub struct Log {
  pub data: Vec<(&'static str, RGB)>,
}

impl Log {
  ///
  /// Get a new, empty log
  ///
  pub fn new() -> Self {
    Log { data: vec![] }
  }

  ///
  /// Get a range of the last n items added to the log
  /// 
  /// The intention of this is that the range is the interated over, and then used as indices
  /// to read the log data
  ///
  pub fn get_last_n_messages(&self, n: usize) -> &[(&'static str, RGB)] {
    // Basically if there are n items in the log, but we want to get > n items, we
    // should make sure rust doesn't have some sort of underflow error
    if n > self.data.len() {
      return &self.data[0..self.data.len()];
    } else {
      return &self.data[(self.data.len() - n)..self.data.len()];
    }
  }

  ///
  /// Push new data onto the log stack
  ///
  pub fn push(&mut self, message: (&'static str, RGB)) {
    self.data.push(message);
  }
}

// Make a mutex availible
lazy_static! {
  pub static ref GlobalLog: Mutex<Log> = Mutex::new(Log::new());
}

/// This macro automates the log mutex process. This whole thing is pretty crazy
/// Oviously if any panics occur here then the mutex becomes poisoned
#[macro_export]
macro_rules! log {
  ($msg:expr) => {{
    // Import it's own lazy static ref
    use self::log::GlobalLog;
    // Lock the mutex
    let mut log = GlobalLog.lock().unwrap();
    // Push the message
    // Highly implies a correct expression for the push arguments are being supplied
    log.push($msg);
    // Drop the reference
    drop(log);
  }};
}

