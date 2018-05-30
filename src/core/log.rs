//!
//! Global, mutable log.
//!

#![allow(non_upper_case_globals)] 

use std::sync::Mutex;
use std::ops::Range;

use core::renderer::RGB;

// A log just wraps some data pretty much
pub struct Log {
  pub data: Vec<(&'static str, RGB)>
}

impl Log {

  // New log. Filler data
  pub fn new() -> Log {
    Log { data: vec![
      ("mmmm", RGB(255, 255, 0)), 
      ("gotta", RGB(255, 0, 255)), 
      ("love", RGB(0, 255, 255)), 
      ("that", RGB(0, 255, 0)), 
      ("rust", RGB(255, 255, 255))
    ] }
  }

  // Get a range of the last n items added to the log
  pub fn get_latest_range(&self, n: usize) -> Range<usize> {
    (self.data.len()-n)..self.data.len()
  }

  // Push new data onto the log stack
  pub fn push(&mut self, s: (&'static str, RGB)) {
    self.data.push(s);
  }

}

// Make a mutex availible
lazy_static! {
  pub static ref GlobalLog: Mutex<Log> = Mutex::new(Log::new());
}
