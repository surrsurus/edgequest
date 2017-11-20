const SCENT_CAP : u8 = 50;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Scent {
  pub value: u8,
  pub persistence: u8,
  pub changed: bool
}

impl Scent {

  pub fn update(&mut self) {

    if self.value > 0 {
      if self.persistence > SCENT_CAP {
        if self.value % 2 == 0 {
          self.value -= 2;
        } else {
          self.value -= 1;
        } 
        
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

  pub fn max(&mut self) {
    self.value = 20;
    self.persistence = SCENT_CAP;
  }

  pub fn new() -> Scent {
    Scent { value: 0, persistence: 0, changed: false }
  }

  pub fn set(&mut self, v: u8) {
    if !self.changed {
      self.value = v;
      self.changed = true;
    }
  }

  pub fn inc(&mut self) {
    if self.value < 255 && self.persistence < SCENT_CAP && !self.changed {
      self.value += 1;
    }
  }

  pub fn dec(&mut self) {
    if self.value > 0 {
      self.value -= 1;
    }
  }
  
}