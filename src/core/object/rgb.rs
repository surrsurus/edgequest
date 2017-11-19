use core::tcod::colors;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct RGB(pub u8, pub u8, pub u8);

impl RGB {

  #[inline]
  pub fn to_tcod_color(&self) -> colors::Color {
    colors::Color::new(self.0, self.1, self.2)
  }

}