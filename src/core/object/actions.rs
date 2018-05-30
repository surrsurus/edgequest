//!
//! Enum representing possible actions creatures can take
//!

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Actions {
  // Creature moved
  Move,
  // Creature waited
  Wait,
  // Creature went down
  DownStair,
  // Creature went up
  UpStair,
  // Creature blinked (teleported randomly)
  Blink,
  // Unknown action (Creature did something weird)
  Unknown
}