extern crate rand;
use self::rand::Rng;

use core::renderer::RGB;

use core::world::dungeon::builder::Buildable;
use core::world::dungeon::builder::construct::{Corr, Rect};

use core::world::dungeon::map::{Grid, Pos, tile, Tile};

///
/// Simple dungeon builder
/// 
/// This builder places a number of small rooms (respective to map size)
/// all connected by corridors.
/// 
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Simple {

  pub grid: Grid<Tile>,
  pub w: usize,
  pub h: usize,

  // Private vector to hold rooms
  rooms: Vec<Rect>,
  
  // Private vector to hold floor type
  floor: Tile

}

// Make Simple Indexable
impl std::ops::Index<usize> for Simple {
  type Output = [Tile];
  fn index(&self, idx: usize) -> &Self::Output {
    &self.grid[idx]
  }
}

impl std::ops::IndexMut<usize> for Simple {
  fn index_mut(&mut self, idx: usize) -> &mut [Tile] {
    &mut self.grid[idx]
  }
}

impl std::ops::Index<Pos> for Simple {
  type Output = Tile;
  fn index(&self, idx: Pos) -> &Self::Output {
    &self.grid[idx]
  }
}

impl std::ops::IndexMut<Pos> for Simple {
  fn index_mut(&mut self, idx: Pos) -> &mut Tile {
    &mut self.grid[idx]
  }
}

impl Simple {

  ///
  /// Add rooms to the `rooms` vec and build them on the `grid`
  /// 
  fn add_rooms(&mut self) {

    // Clear rooms
    self.rooms = Vec::<Rect>::new();

    let mut rng = rand::thread_rng();

    // Number of rooms correspond to map size
    let n = (self.w + self.h) / 10;

    for _ in 0..n {

      let x: isize = rng.gen_range(1, self.w as isize - 2);
      let y: isize = rng.gen_range(1, self.h as isize - 2);
      let l: isize = rng.gen_range(5, 20);
      let w: isize = rng.gen_range(5, 20);

      // Check bounds
      if w + x >= self.w as isize|| l + y >= self.h as isize {
        continue;
      } else {
        let r = Rect::new(x, y, l, w);
        self.build_rect(&r);
        self.rooms.push(r);
      }
      
    }
    
  }

  /// 
  /// Build a corridor to the grid
  /// 
  /// Start by moving along the x-axis, then the y-axis
  /// 
  fn build_corr(&mut self, c: &Corr) {

    let mut mover = c.start;

    while mover.x != c.end.x {

      if mover.x < c.end.x {
        mover.x += 1;
      } else if mover.x > c.end.x {
        mover.x -= 1;
      } 

      self[mover] = self.floor.clone();

    }

    while mover.y != c.end.y {

      if mover.y < c.end.y {
        mover.y += 1;
      } else if mover.y > c.end.y {
        mover.y -= 1;
      } 

      self[mover] = self.floor.clone();

    }

  }

  /// 
  /// Build a rectangle to the grid
  /// 
  fn build_rect(&mut self, r: &Rect) {
    for w in 0..r.w {
      for l in 0..r.l {
        self[(w + r.x) as usize][(l + r.y) as usize] = self.floor.clone();
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
  fn connect_rooms(&mut self) {

    for r in 0..self.rooms.len() - 1 {

      let c1 : Pos;
      let c2 : Pos;

      // Wrap around
      if r == self.rooms.len() - 1 {

        c1 = self.rooms[r].center().clone();
        c2 = self.rooms[0].center().clone();

      } else {  

        c1 = self.rooms[r].center().clone();
        c2 = self.rooms[r + 1].center().clone();

      }

      self.build_corr(&Corr::new(c1, c2));

    }

  }


  /// 
  /// Return a new `Simple`
  /// 
  pub fn new(grid: Grid<Tile>) -> Self {

    // Make a new dungeon with our fresh grid of size `w` by `h`
    let s = Simple { 
      grid: grid.clone(), 
      rooms: Vec::<Rect>::new(), 
      w: grid.len(), 
      h: grid[0].len(),
      // Floor type. Doesn't need to be changed right now, after all this is the 'simple' dungeon builder
      floor: Tile::new("Floor", ' ', RGB(7, 7, 7),  RGB(0, 0, 0), tile::Type::Floor(tile::Floor::Normal))
    };
    
    return s;

  }

}

impl Buildable for Simple {

  type Output = Tile;

  fn build(&mut self) -> Grid<Tile> {

    // Generate the dungeon
    self.add_rooms();
    self.connect_rooms();

    return self.grid.clone();

  }

}