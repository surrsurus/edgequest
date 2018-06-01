//! 
//! A module for loading a YAML config file.
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

  pub screen_width: isize,
  pub screen_height: isize,

  pub map_width: isize,
  pub map_height: isize,

  pub console_height: isize,
  pub panel_width: isize,

  pub fullscreen: bool,

  pub fontpath: String,

  pub fonttype: String,
  pub fontlayout: String,

  pub renderer: String

}

///
/// NOTE: Pretty sure this whole thing is deprecated. Needs to change
///
/// Load configuration data from a path. returns a `Config` struct.
/// 
/// This function expects to be passed in a valid YAML file that has YAML for each attribute
/// in `Config`.
///
/// * `path` - Path to desired YAML file.
/// 
/// # What data should be in your configuration file
/// 
/// * `screen_width` - screen_width of the window in characters.
/// * `screen_height` - screen_height of the window in characters.
/// * `fullscreen` - Determines whether or not game will start in fullscreen mode.
/// * `fontpath` - Path to desired font to use.
/// * `fonttype` - Type of font, either Default or Greyscale.
/// * `fontlayout` - Layout of font. Either Tcod, AsciiInRow, or AsciiInCol.
/// * `renderer` - Desired renderer to use. Either SDL, GLSL, or OpenGL.
/// 
/// # Panics
/// 
/// This function will panic if:
/// 
/// * The path is invalid
/// * The file is invalid
/// * The file is not a YAML file
/// * The file is missing YAML for any attribute of `Config`
/// * The YAML for each attribute is not the correct type
/// * The YAML for fonttype, fontlayout, or renderer are not in their tcod enums
/// 
/// This is definitely a very touchy function but it is important that there are no errors
/// with the configuration file because initializing the root console depends heavily on it.
///  
pub fn load(path: &str) -> Config {

  // Load file to String
  let mut file = File::open(path).expect("Unable to open");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Problem reading file");

  let ds_cfg: Config = serde_yaml::from_str(&contents).unwrap();

  return ds_cfg;

}