/// 
/// Holds a position and a character.
/// 
/// Used for everything, basically, since all things in order to
/// be rendered need 1) a place to be rendered on the screen
/// and 2) a character to represent them on the screen.
///
/// * `pos` - `Pos` representing where the entity is on the map
/// * `glyph` - Character to represent entity on screen
/// 
pub trait Entity {

  #[inline]
  fn get_name(&self) -> String;

  #[inline]
  fn set_name(&mut self, name: String);

}