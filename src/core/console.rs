//!
//! Literally not used at all, pretty sure working on UI stuff is definitely not happening soon.
//!

struct ConsoleString {
  content: &'static str
  fg: RGB,
  bg: RGB
}

struct Console {
  buffer: Vec<ConsoleString>,
  length: i32
}

impl Console {

  pub fn new() -> Console {
    
  }

  pub fn write(&mut self) {

  }

}