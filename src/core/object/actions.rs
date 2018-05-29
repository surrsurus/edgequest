
///
/// Enum representing possible actions the player can take
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Actions {
  // Player moved
  Move,
  // Player waited
  Wait,
  // Player went down
  DownStair,
  // Player went up
  UpStair,
  //
  Blink,
  // Unknown action (Player pressed unbound key)
  Unknown
}