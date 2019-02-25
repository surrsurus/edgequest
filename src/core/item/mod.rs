
use core::creature::Creature;
use core::renderer::{Renderable, RGB};
use core::time::Time;
use core::world::dungeon::map::{self, Pos, Tile};

#[derive(Clone)]
pub enum Money {
  Copper,
  Silver,
  Electrum,
  Gold,
  Quartz,
  Platinum,
  Mithril,
  Scale,
  Onyx,
  Tourmaline,
  Emerald,
  Ruby,
  Sapphire,
  Topaz,
  Diamond,
}

pub fn money_value(money: &Money) -> f32 {
  match money {
    Money::Copper => 0.01,
    Money::Silver => 0.1,
    Money::Electrum => 0.5,
    Money::Gold | Money::Quartz => 1.0,
    Money::Platinum => 2.0,
    Money::Mithril | Money::Scale => 5.0,
    Money::Onyx | Money::Tourmaline => 7.0,
    Money::Emerald | Money::Ruby | Money::Sapphire | Money::Topaz => 10.0,
    Money::Diamond => 100.0
  }
}

#[derive(Clone)]
pub enum ItemProperty {
  // Money is an interesting property because it should go right to a player's wallet
  Money(Money)
}

///
/// Item struct
///
#[derive(Clone)]
pub struct Item {
  name: &'static str,
  glyph: char,
  // Items can potentially be in something's inventory
  pub pos: Pos,
  fg: RGB,
  bg: RGB,

  // Items could have quantity like stacks of arrows, portions of food, liters of water etc
  pub quantity: isize,

  // Item property
  pub property: ItemProperty
}

impl Item {
  ///
  /// Return a new `Item`
  ///
  #[inline]
  pub fn new(name: &'static str, glyph: char, pos: Pos, fg: RGB, bg: RGB, quantity: isize, property: ItemProperty) -> Self {
    Item {
      name: name,
      glyph: glyph, 
      pos: pos, 
      fg: fg, 
      bg: bg,
      quantity: quantity,
      property: property
    }
  }
}

///
/// Implement the `Renderable` trait for `Item`, mostly just getters and setters
///
impl Renderable for Item {

  #[inline]
  fn get_bg(&self) -> RGB {
    self.bg
  }

  #[inline]
  fn get_fg(&self) -> RGB {
    self.fg
  }

  #[inline]
  fn get_glyph(&self) -> char {
    self.glyph
  }

  #[inline]
  fn get_id(&self) -> &'static str {
    self.name.clone()
  }

  #[inline]
  fn set_bg(&mut self, bg: RGB) {
    self.bg = bg;
  }

  #[inline]
  fn set_fg(&mut self, fg: RGB) {
    self.fg = fg;
  }

  #[inline]
  fn set_glyph(&mut self, glyph: char) {
    self.glyph = glyph
  }

  #[inline]
  fn set_id(&mut self, name: &'static str) {
    self.name = name;
  }

}

impl Time for Item {

  fn take_turn(&mut self, _map: &map::Grid<Tile>, _player: &Creature) {

  }

}