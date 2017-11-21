const SCENT_CAP : u8 = 150;

///
/// Scent struct. Holds values for the ScentMap, used by monsters to track the player.
/// 
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Scent {
  pub value: u8,
}

impl Scent {

  ///
  /// Return a new `Scent`
  /// 
  pub fn new() -> Scent {
    Scent { value: 0 }
  }

  pub fn as_f32(&self) -> f32 {
    self.value as f32
  }

  pub fn inc(&mut self, i: u8) {
    if self.value < 255 - i {
      self.value = i;
    }
  }

  pub fn filter(&self) -> f32 {
    if self.value == 0 {
      0.1
    } else {
      1.0
    }
  }
  
}