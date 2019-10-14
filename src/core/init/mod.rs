//! 
//! Initialize tcod elements.
//! 
//! Depends on the `config` module.
//! 

// We need tcod::Console to keep our consoles in scope
#[allow(unused_imports)]
use core::tcod::{Console, console};

pub mod config;
use self::config::Config;

mod init_tests;

// So for a while each function here loaded the config on it's own. And I thought to myself, "hm, is it possible for this
// to only be loaded once?"
//
// Well then I remembered the log system.
//
// What this does is essentially just creates a private static reference a single time to a config struct loaded from `config`
// on runtime thanks to lazy_static. I can then just reference elements of the config object via this and not have to redundantly
// load the file
lazy_static! {
  static ref CFG : Config = config::load("config/cfg.yml");
}

///
/// Initialize the root console.
/// 
/// Returns a console that is meant to be used as the root console. Console
/// settings depend on `config::load()`.
/// 
pub fn root() -> console::Root {

  // Match fonttype based on the FontType enum
  let fonttype = match CFG.fonttype.as_str() {
    "Default" => console::FontType::Default,
    "Greyscale" => console::FontType::Greyscale,
    _ => panic!("Bad font type: {}", CFG.fonttype)
  };

  // Match fontlayout based on the FontLayout enum
  let fontlayout = match CFG.fontlayout.as_str() {
    "Tcod" => console::FontLayout::Tcod,
    "AsciiInRow" => console::FontLayout::AsciiInRow,
    "AsciiInCol" => console::FontLayout::AsciiInCol,
    _ => panic!("Bad font type: {}", CFG.fontlayout)
  };

  // Match renderer based on the Renderer enum
  let renderer = match CFG.renderer.as_str() {
    "SDL" => console::Renderer::SDL,
    "GLSL" => console::Renderer::GLSL,
    "OpenGL" => console::Renderer::OpenGL,
    _ => panic!("Bad font type: {}", CFG.renderer)
  };

  // Return a Root console
  return console::Root::initializer()
    .size(CFG.screen_width as i32, CFG.screen_height as i32)
    .title("edgequest")
    .fullscreen(CFG.fullscreen)
    // Not sure why this one needs to be cloned,
    // probably because font completely passes the reference to the console itself
    // and we need to preserve the static ref otherwise it's pointless
    .font(CFG.fontpath.clone(), fontlayout)
    .font_type(fonttype)
    .renderer(renderer)
    .init();

}

///
/// Get map dimensions as a tuple
/// 
/// We return this as a tuple and not a Pos because ideally the init should purely only rely on
/// the filesystem loading and tcod for initializing the console
/// 
pub fn map_dimensions() -> (isize, isize) {
  (CFG.map_width, CFG.map_height)
}

///
/// Get console height
///
pub fn console_height() -> isize {
  CFG.console_height
}

///
/// Get panel width
///
pub fn panel_width() -> isize {
  CFG.panel_width
}

///
/// Get wizard mode
///
pub fn wizard() -> bool {
  CFG.wizard
}

///
/// Get debug mode
///
pub fn debug() -> bool {
  CFG.debug
}