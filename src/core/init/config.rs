//! 
//! A module for loading a YAML config file with serde.
//! 

// Serde
extern crate serde;
extern crate serde_yaml;

// Use to read files
use std::fs::File;
use std::io::prelude::*;

///
/// A struct to hold data gathered from a config.yml file. You should not need to create your own,
/// instead, get a filled out struct from `load()`
/// 
/// # What data should be in your configuration file
/// 
/// See `load()`
/// 
/// # Determining font settings
/// 
/// Firstly, pick a font from the `fonts` directory. 
/// Once you have a font you like, add it to the `fontpath` 
/// in the configuration file, so it looks like `fontpath: fonts/yourfont.png`.
/// 
/// The next step is figuring out your FontType and FontLayout. 
/// Your font file will be in the form `name_FontType_FontLayout`. 
/// Here is how they line up.
/// 
/// ## FontType
/// * `aa` - Default
/// * `gs` - Greyscale
/// 
/// ## FontLayout
/// * `tc` - Tcod
/// * `ro` - AsciiInRow
/// * `as` - AsciiInCol
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {

  // Size of physical screen
  pub screen_width: isize,
  pub screen_height: isize,

  // Size of default dungeon maps
  pub map_width: isize,
  pub map_height: isize,

  // Size of the console at the bottom of the screen
  pub console_height: isize,
  
  // Size of the panel on the right of the screen
  pub panel_width: isize,

  // Toggles fullscreen
  pub fullscreen: bool,

  // Determines relative path to the font to be used
  pub fontpath: String,

  // Determines type of font
  pub fonttype: String,

  // Determines layout of font
  pub fontlayout: String,

  // Determines renderer to be used
  pub renderer: String,

  // Wizard mode
  pub wizard: bool,

  // Debug logs
  pub debug: bool

}

///
/// Load configuration data from a path. 
/// Deserializes data from the file to a `Config` struct with serde.
///
/// Serde typically will fail on its own if there is any malformed data,
/// but in instance of specific strings it should be up to `init` to verify them
/// 
pub fn load(path: &str) -> Config {

  // Load file to String
  let mut file = File::open(path).expect("Unable to open");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Problem reading file");

  // Deserialize content to Config
  let ds_cfg: Config = serde_yaml::from_str(&contents).unwrap();

  return ds_cfg;

}