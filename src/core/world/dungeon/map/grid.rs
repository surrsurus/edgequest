///
/// Hold a 2D vector of `T`s
/// 
/// Primarily used for dungeon generation and cellular automatons, also it looks better than `Vec<Vec<T>>` and is more
/// intuitive as to what it actually represents
/// 
pub type Grid<T> = Vec<Vec<T>>;