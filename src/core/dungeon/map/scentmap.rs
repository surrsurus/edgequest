use core::dungeon::map::{Grid, Scent, Tile};
use core::dungeon::Dungeon;
use core::object::Fighter;

pub type ScentMap = Grid<Scent>;

impl ScentMap {

  fn avg_of_neighbors(&self, buffer: &ScentMap, x: usize, y: usize) -> f32 {

    (
      (buffer.0[x - 1][y].as_f32() +
      buffer.0[x + 1][y].as_f32() +
      buffer.0[x][y - 1].as_f32() +
      buffer.0[x][y + 1].as_f32() +
      buffer.0[x + 1][y + 1].as_f32() +
      buffer.0[x - 1][y - 1].as_f32() +
      buffer.0[x + 1][y - 1].as_f32() +
      buffer.0[x - 1][y + 1].as_f32()) / 
      (((buffer.0[x - 1][y].filter() +
      buffer.0[x + 1][y].filter() +
      buffer.0[x][y - 1].filter() +
      buffer.0[x][y + 1].filter() +
      buffer.0[x + 1][y + 1].filter() +
      buffer.0[x - 1][y - 1].filter() +
      buffer.0[x + 1][y - 1].filter() +
      buffer.0[x - 1][y + 1].filter()) + 0.05) * (255.0/256.0))
    ) 

  }
  
  pub fn update(&mut self, grid: &Grid<Tile>) {

    let width : usize = self.0.len();
    let height : usize = self.0[0].len();

    let mut buffer : ScentMap = Grid(vec![]);
    for x in 0..width {
      let mut scent_vec = Vec::<Scent>::new();
      for y in 0..height {
        scent_vec.push(self.0[x][y].clone());
      }
      buffer.0.push(scent_vec);
    }

    for x in 0..width {
      for y in 0..height {
         if x + 2 > 0 && x + 2 < width && y > 1 && y + 2 < height && !grid.0[x][y].blocks {
          self.0[x][y].value = self.avg_of_neighbors(&buffer, x, y) as u8;
         }
      }
    }

  }

}