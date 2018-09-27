//!
//! Enum representing possible actions creatures can take
//!

///
/// All actions are meant to be an intended state for a creature to be in
///
/// This is also used by the main game engine to percieve what the player is doing
/// at any given moment in time and make choices on what to do based on it
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Actions {
  // Creature moved
  Move,
  // Creature waited
  Wait,
  // NOTE: So right now the only creature that can "go down" or "go up" is the player
  // but now I'm thinking about monsters that decide to traverse the floors at their discretion...
  // Creature went down
  DownStair,
  // Creature went up
  UpStair,
  // Creature blinked (teleported randomly)
  Blink,
  // Creature Talked
  Talk,
  // Creature died
  Die,
  // Unknown action (Creature did something weird)
  Unknown
}