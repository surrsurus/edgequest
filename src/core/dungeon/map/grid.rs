///
/// Hold a 2D vector of `T`s
/// 
/// Primarily used for dungeon generation and cellular automatons
/// The idea of this is to generate your maps onto a `Grid` and then pass it back
/// up as a `collapse()`d grid.
/// 
#[deny(warnings)]
pub type Grid<T : Clone> = Vec<Vec<T>>;