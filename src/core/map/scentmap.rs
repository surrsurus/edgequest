use core::map::{Floor, Scent};

pub struct ScentMap(pub Vec<Vec<Scent>>);

impl ScentMap {
  
  pub fn update_scent_map(&mut self, floor: &Floor) {

    // self.0[self.player.pos.x as usize][self.player.pos.y as usize].max();

    // for nx in -1..2 {
    //   for ny in -1..2 {
    //     if self.player.pos.x - nx > 0 && self.player.pos.x - nx < floor.width as isize &&self.player.pos.y - ny > 0 && self.player.pos.y - ny < floor.height as isize && !floor.tile_vec.0[(self.player.pos.x - nx) as usize][(self.player.pos.y - ny) as usize].blocks{
    //       self.0[(self.player.pos.x - nx) as usize][(self.player.pos.y - ny) as usize].ma();
    //     }
    //   }
    // }

    for x in 0..floor.width {
      for y in 0..floor.height {
        self.0[x][y].update();
        if self.0[x][y].value > 4 {
          if x + 1 > 0 && x + 1 < floor.width && y > 0 && y < floor.height && !floor.tile_vec.0[(x + 1) as usize][(y) as usize].blocks{
            if self.0[(x + 1) as usize][(y) as usize].value < self.0[x][y].value {
              self.0[(x + 1) as usize][(y) as usize].value = self.0[x][y].value - 1;
            }
          }
          if x - 1 > 0 && x - 1 < floor.width && y > 0 && y < floor.height && !floor.tile_vec.0[(x - 1) as usize][(y) as usize].blocks{
            if self.0[(x - 1) as usize][(y) as usize].value < self.0[x][y].value {
              self.0[(x - 1) as usize][(y) as usize].value = self.0[x][y].value - 1;
            }
          }
          if x > 0 && x < floor.width && y - 1 > 0 && y - 1 < floor.height && !floor.tile_vec.0[(x) as usize][(y - 1) as usize].blocks{
            if self.0[(x) as usize][(y - 1) as usize].value < self.0[x][y].value {
              self.0[(x) as usize][(y - 1) as usize].value = self.0[x][y].value - 1;
            }
          }
          if x > 0 && x < floor.width && y + 1 > 0 && y + 1 < floor.height && !floor.tile_vec.0[(x) as usize][(y + 1) as usize].blocks{
            if self.0[(x) as usize][(y + 1) as usize].value < self.0[x][y].value {
              self.0[(x) as usize][(y + 1) as usize].value = self.0[x][y].value - 1;
            }
          }
        }
      }
    }
  }

}