//! 
//! A module for loading a YAML config file.
//! 

// Use to parse YAML
extern crate yaml_rust;
use self::yaml_rust::YamlLoader;

// Use to read files
use std::fs::File;
use std::io::prelude::*;

// Use this for three enums that are used in the Config struct
use tcod::console;

///
/// A struct to hold data gathered from a config.yml file. You should not need to create your own,
/// instead, get a filled out struct from [`load()`](fn.load.html)
/// 
/// # What data should be in your configuration file
/// 
/// See [`load()`](fn.load.html)
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
  // i32 because of tcod
  pub width: i32,
  pub height: i32,
  pub fullscreen: bool,
  pub fontpath: String,
  pub fonttype: console::FontType,
  pub fontlayout: console::FontLayout,
  pub renderer: console::Renderer,
}

///
/// Load configuration data from a path. returns a [`Config`](struct.Config.html) struct.
/// 
/// This function expects to be passed in a valid YAML file that has YAML for each attribute
/// in [`Config`](struct.Config.html).
/// 
/// * `path` - Path to desired YAML file.
/// 
/// # What data should be in your configuration file
/// 
/// * `width` - Width of the window in characters.
/// * `height` - Height of the window in characters.
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
/// * The file is missing YAML for any attribute of [`Config`](struct.Config.html)
/// * The YAML for each attribute is not the correct type
/// * The YAML for fonttype, fontlayout, or renderer are not in their tcod enums
/// 
/// This is definitely a very touchy function but it is important that there are no errors
/// with the configuration file because initializing the root console depends heavily on it.
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

    // Width and height can only be read as i64s (why???) so we use as i32
    // to convert them down
    width: cfg["width"].as_i64().unwrap() as i32,
    height: cfg["height"].as_i64().unwrap() as i32,

    // Font path should be a String so it doesnt have a lifetime
    fontpath: cfg["fontpath"].as_str().unwrap().to_string(),

    // Font path should be a String so it doesnt have a lifetime
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

// Load test should fail on invalid paths
#[test]
#[should_panic]
fn test_bad_path_load() {
  load("bad path");
}

// Load test should fail on invalid file types
#[test]
#[should_panic]
fn test_bad_file_load() {
  load("src/main.rs");
}

// Load test should fail on files with valid yml that do not fit the Config struct types
#[test]
#[should_panic]
fn test_bad_types_load() {
  load("tests/bad_types.yml");
}

// Load test should fail on files without all the proper fields
#[test]
#[should_panic]
fn test_missing_load() {
  load("tests/missing.yml");
}

// Load test should fail on files with just invalid fields
#[test]
#[should_panic]
fn test_invalid_breaking_load() {
  load("tests/invalid_breaking.yml");
}

// ... However load test should not fail on files with extra invalid fields
#[test]
fn test_invalid_nonbreaking_load() {
  load("tests/invalid_nonbreaking.yml");
}

// Load passes with no problems on proper yml files
#[test]
fn test_good_load() {
  load("config/cfg.yml");
}