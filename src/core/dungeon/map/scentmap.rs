use core::dungeon::map::{Grid, Scent, Tile};
use core::dungeon::Dungeon;
use core::object::Fighter;

pub type ScentMap = Grid<Scent>;

impl ScentMap {
  
  pub fn update(&mut self, grid: &Grid<Tile>, player: &Fighter) {

    let width = self.0.len();
    let height = self.0[0].len();

    for nx in -1..2 {
      for ny in -1..2 {
        if player.pos.x - nx > 0 && player.pos.x - nx < width as isize &&player.pos.y - ny > 0 && player.pos.y - ny < height as isize && !grid.0[(player.pos.x - nx) as usize][(player.pos.y - ny) as usize].blocks{ 
          self.0[(player.pos.x - nx) as usize][(player.pos.y - ny) as usize].max();
        }
      }
    }

    for x in 0..width {
      for y in 0..height {
        self.0[x][y].update();
        if self.0[x][y].value > 3 {
          if x + 1 > 0 && x + 1 < width && y > 0 && y < height && !grid.0[(x + 1) as usize][(y) as usize].blocks{
            if self.0[(x + 1) as usize][(y) as usize].value < self.0[x][y].value {
              self.0[(x + 1) as usize][(y) as usize].inc();
              self.0[x][y].dec();
            }
          }
          if x - 1 > 0 && x - 1 < width && y > 0 && y < height && !grid.0[(x - 1) as usize][(y) as usize].blocks{
            if self.0[(x - 1) as usize][(y) as usize].value < self.0[x][y].value {
              self.0[(x - 1) as usize][(y) as usize].inc();
              self.0[x][y].dec();
            }
          }
          if x > 0 && x < width && y - 1 > 0 && y - 1 < height && !grid.0[(x) as usize][(y - 1) as usize].blocks{
            if self.0[(x) as usize][(y - 1) as usize].value < self.0[x][y].value {
              self.0[(x) as usize][(y - 1) as usize].inc();
              self.0[x][y].dec();
            }
          }
          if x > 0 && x < width && y + 1 > 0 && y + 1 < height && !grid.0[(x) as usize][(y + 1) as usize].blocks{
            if self.0[(x) as usize][(y + 1) as usize].value < self.0[x][y].value {
              self.0[(x) as usize][(y + 1) as usize].inc();
              self.0[x][y].dec();
            }
          }
        }
      }
    }
  }

}