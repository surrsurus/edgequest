//! 
//! Initialize tcod elements.
//! 
//! Depends on the `config` module.
//! 

// We need tcod::Console to keep our consoles in scope
#[allow(unused_imports)]
use core::tcod::{Console, console};

pub mod config;

mod init_tests;

use core::object::Pos;

///
/// Initialize the root console.
/// 
/// Returns a console that is meant to be used as the root console. Console
/// settings depend on `config::load()`.
/// 
pub fn root() -> console::Root {

  let cfg = config::load("config/cfg.yml");

  return console::Root::initializer()
    .size(cfg.screen_width, cfg.screen_height)
    .title("EQ")
    .fullscreen(cfg.fullscreen)
    .font(cfg.fontpath, cfg.fontlayout)
    .font_type(cfg.fonttype)
    .renderer(cfg.renderer)
    .init();

}

pub fn map_dimensions() -> Pos {
  
  let cfg = config::load("config/cfg.yml");

  return Pos::new(cfg.map_width, cfg.map_height);

}