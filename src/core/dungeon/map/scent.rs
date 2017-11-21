const SCENT_CAP : u8 = 150;

///
/// Scent struct. Holds values for the ScentMap, used by monsters to track the player.
/// 
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Scent {
  pub value: u8,
  pub persistence: u8,
  pub changed: bool
}

impl Scent {

  ///
  /// Increment the scent
  /// 
  pub fn inc(&mut self) {
    if self.value < 255 && self.persistence < SCENT_CAP && !self.changed {
      self.value += 1;
    }
    // self.persistence += 1;
  }

  ///
  /// Update the scent
  /// 
  pub fn update(&mut self) {

    if self.value > 0 {

      if self.persistence > SCENT_CAP {

        // if self.value % 2 == 0 {

        //   self.value -= 2;

        // } else {

        //   self.value -= 1;

        // } 

        self.value -= 1;
        
        if self.value == 0 {

          self.persistence = 0;

        }

      } else {

        self.persistence += 1;

      }
      
    }

    if self.changed {
      self.changed = false;
    }

  }

  ///
  /// Max out the scent
  /// 
  pub fn max(&mut self) {
    self.value = 100;
    self.persistence = SCENT_CAP / 2;
  }

  ///
  /// Return a new `Scent`
  /// 
  pub fn new() -> Scent {
    Scent { value: 0, persistence: 0, changed: false }
  }

  ///
  /// Set the value of the scent
  /// 
  pub fn set(&mut self, v: u8) {
    if !self.changed {
      self.value = v;
      self.changed = true;
    }
  }



  /// 
  /// Decrement the scent
  /// 
  pub fn dec(&mut self) {
    if self.value > 0 {
      self.value -= 1;
    }
  }
  
}