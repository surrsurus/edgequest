///
/// Hold a 2D vector of `T`s
/// 
/// Primarily used for dungeon generation and cellular automatons
/// The idea of this is to generate your maps onto a `Grid` and then pass it back
/// up as a `collapse()`d grid.
/// 
pub type Grid<T> = Vec<Vec<T>>;