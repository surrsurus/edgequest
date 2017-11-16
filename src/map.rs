use object::{Entity};
use tile::Tile;

pub struct Map {
    pub length: usize,
    pub height: usize,
    pub tile_vec: Vec<Tile>,
    pub entity_vec: Vec<Entity>,
}