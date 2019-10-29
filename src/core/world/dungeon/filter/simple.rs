extern crate rand;
use self::rand::Rng;

use core::renderer::RGB;

use super::Filter;

use super::map::construct::{Corr, Rect};
use super::map::{Grid, Measurable, Pos, tile, Tile};

///
/// Simple dungeon builder
/// 
/// This builder places a number of small rooms (respective to map size)
/// all connected by corridors.
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Simple {

  pub w: usize,
  pub h: usize,

  // Private vector to hold rooms
  rooms: Vec<Rect>,
  
  // Private vector to hold floor type
  floor: Tile

}

impl Simple {

  ///
  /// Add rooms to the `rooms` vec and build them on the `grid`
  /// 
  fn add_rooms(&mut self, grid: &mut Grid<Tile>) {

    // Clear rooms
    self.rooms = Vec::<Rect>::new();

    let mut rng = rand::thread_rng();

    // Number of rooms correspond to map size
    let n = (self.w + self.h) / 10;

    for _ in 0..n {

      let x: isize = rng.gen_range(1, self.w as isize - 2);
      let y: isize = rng.gen_range(1, self.h as isize - 2);
      let h: isize = rng.gen_range(5, 20);
      let w: isize = rng.gen_range(5, 20);

      // Check bounds
      if w + x >= self.w as isize|| h + y >= self.h as isize {
        continue;
      } else {
        let rect = Rect::new(x, y, h, w);
        self.build_rect(&rect, grid);
        self.rooms.push(rect);
      }
      
    }
    
  }

  /// 
  /// Build a corridor to the grid
  /// 
  /// Start by moving along the x-axis, then the y-axis
  /// 
  fn build_corr(&mut self, c: &Corr, grid: &mut Grid<Tile>) {

    let mut mover = c.start;

    while mover.x != c.end.x {

      if mover.x < c.end.x {
        mover.x += 1;
      } else if mover.x > c.end.x {
        mover.x -= 1;
      } 

      grid[mover] = self.floor.clone();

    }

    while mover.y != c.end.y {

      if mover.y < c.end.y {
        mover.y += 1;
      } else if mover.y > c.end.y {
        mover.y -= 1;
      } 

      grid[mover] = self.floor.clone();

    }

  }

  /// 
  /// Build a rectangle to the grid
  /// 
  fn build_rect(&mut self, rect: &Rect, grid: &mut Grid<Tile>) {
    for w in 0..rect.w {
      for h in 0..rect.h {
        grid[(w + rect.x) as usize][(h + rect.y) as usize] = self.floor.clone();
      }
    }
  }

  /// 
  /// Connect rooms by making `Corr`s then build them.
  /// 
  /// Essentially we connect each room in `rooms` to the next room in the
  /// vector, once we run out we wrap it back around, this in theory
  /// creates a cyclical dungeon with no deadends, but it doesn't happen
  /// consistently due to the way corridors are built (which is a good thing).
  /// 
  fn connect_rooms(&mut self, grid: &mut Grid<Tile>) {

    for room_idx in 0..self.rooms.len() - 1 {

      let start : Pos;
      let end : Pos;

      // Wrap around
      if room_idx == self.rooms.len() - 1 {

        start = self.rooms[room_idx].center();
        end = self.rooms[0].center();

      } else {  

        start = self.rooms[room_idx].center();
        end = self.rooms[room_idx + 1].center();

      }

      self.build_corr(&Corr::new(start, end), grid);

    }

  }


  /// 
  /// Return a new `Simple`
  /// 
  pub fn new(grid: &Grid<Tile>) -> Self {

    // Make a new dungeon with our fresh grid of size `w` by `h`
    let simple = Simple { 
      rooms: Vec::<Rect>::new(), 
      w: grid.width(), 
      h: grid.height(),
      // Floor type. Doesn't need to be changed right now, after all this is the 'simple' dungeon builder
      floor: Tile::new("Floor", ' ', RGB(7, 7, 7),  RGB(0, 0, 0), tile::Type::Floor(tile::Floor::Normal))
    };
    
    return simple;

  }

}

impl Filter for Simple {

  type Output = Tile;

  fn apply(&mut self, grid: &mut Grid<Self::Output>) {

    debugln!("simple", "building a simple dungeon...");

    // Generate the dungeon
    self.add_rooms(grid);
    self.connect_rooms(grid);

  }

}