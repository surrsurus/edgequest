//! 
//! A module for loading a YAML config file.
//! 
//! NOTE: Probably will be deprecated in favor of using serde
//! 

// Use to parse YAML
extern crate yaml_rust;
use self::yaml_rust::YamlLoader;

// Use to read files
use std::fs::File;
use std::io::prelude::*;

// Use this for three enums that are used in the Config struct
use core::tcod::console;

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
#[derive(Clone)]
pub struct Config {

  pub screen_width: isize,
  pub screen_height: isize,

  pub map_width: isize,
  pub map_height: isize,

  pub console_height: isize,

  pub fullscreen: bool,

  pub fontpath: String,

  pub fonttype: console::FontType,
  pub fontlayout: console::FontLayout,

  pub renderer: console::Renderer

}

///
/// Hold possible return errors from `load()`
///
/// NOTE: We want to use this in a future version of `load()`
///
// enum ConfigError {
//   BadFontType,
//   BadFontLayout,
//   BadRenderer
// }

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
/// NOTE: *Definitely* want to make sure this returns a Result<Config, ConfigError>
///
pub fn load(path: &str) -> Config {

  // Q: Why not use SerDe for this?
  // A: Tcod enums dont derive serialize/deserialize

  // Load file to String
  let mut file = File::open(path).expect("Unable to open");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Problem reading file");

  // ... But we actually need a &str. Wish YamlLoader had
  // a load_from_reader.
  let cfgs = YamlLoader::load_from_str(&contents).unwrap();
  // Apparently YamlLoader has this weird quirk where it doesn't do
  // what you think it would do.
  let cfg = &cfgs[0];

  // Return a Config struct
  return Config { 

    // screen_width and screen_height can only be read as i64s so we use as isize
    // to convert them down
    screen_width: cfg["screen_width"].as_i64().unwrap() as isize,
    screen_height: cfg["screen_height"].as_i64().unwrap() as isize,

    map_width: cfg["map_width"].as_i64().unwrap() as isize,
    map_height: cfg["map_height"].as_i64().unwrap() as isize,

    console_height: cfg["console_height"].as_i64().unwrap() as isize,

    // Font path should be a String so it doesnt have a lifetime
    fontpath: cfg["fontpath"].as_str().unwrap().to_string(),

    // Fullscreen mode
    fullscreen: cfg["fullscreen"].as_bool().unwrap(),

    // Match fonttype based on the FontType enum
    fonttype: match cfg["fonttype"].as_str().unwrap() {
      "Default" => console::FontType::Default,
      "Greyscale" => console::FontType::Greyscale,
      _ => panic!("Wrong FontType in yaml file!")
    },

    // Match fontlayout based on the FontLayout enum
    fontlayout: match cfg["fontlayout"].as_str().unwrap() {
      "Tcod" => console::FontLayout::Tcod,
      "AsciiInRow" => console::FontLayout::AsciiInRow,
      "AsciiInCol" => console::FontLayout::AsciiInCol,
      _ => panic!("Wrong FontLayout in yaml file!")
    },

    // Match renderer based on the Renderer enum
    renderer: match cfg["renderer"].as_str().unwrap() {
      "SDL" => console::Renderer::SDL,
      "GLSL" => console::Renderer::GLSL,
      "OpenGL" => console::Renderer::OpenGL,
      _ => panic!("Wrong Renderer in yaml file!")
    }

  }

}