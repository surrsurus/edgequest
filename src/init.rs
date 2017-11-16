//! 
//! Initialize tcod elements.
//! 
//! Depends on the [`config`](../config/index.html) module.
//! 

// We need tcod::Console to keep our consoles in scope
#[allow(unused_imports)]
use tcod::Console;
use tcod::console;

use config;

///
/// Initialize the root console.
/// 
/// Returns a console that is meant to be used as the root console. Console
/// settings depend on [`config::load()`](../config/fn.load.html).
/// 
pub fn root() -> console::Root {

  let cfg = config::load("config/cfg.yml");

  return console::Root::initializer()
    .size(cfg.width, cfg.height)
    .title("EQ")
    .fullscreen(cfg.fullscreen)
    .font(cfg.fontpath, cfg.fontlayout)
    .font_type(cfg.fonttype)
    .renderer(cfg.renderer)
    .init();

}

// Test the root console creation
#[test]
fn test_root() {

  let cfg = config::load("config/cfg.yml");
  let root = root();

  assert_eq!(root.width(), cfg.width);
  assert_eq!(root.height(), cfg.height);
  assert_eq!(root.is_active(), true);
  assert_eq!(root.is_fullscreen(), cfg.fullscreen);

}