extern crate rand;
use self::rand::{thread_rng, Rng};

use core::dungeon::builder::Buildable;
use core::dungeon::builder::construct::{Corr, Rect};

use core::dungeon::map::{Grid, Tile};

///
/// Simple dungeon builder
/// 
/// This builder places a number of small rooms (respective to map size)
/// all connected by corridors.
/// 
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Simple {

  pub grid: Grid<Tile>,
  pub w: usize,
  pub h: usize,

  // Privatre vector to hold rooms
  rooms: Vec<Rect>,

}

impl Simple {

  ///
  /// Add rooms to the `rooms` vec and build them on the `grid`
  /// 
  fn add_rooms(&mut self) {

    // Clear rooms
    self.rooms = Vec::<Rect>::new();

    let mut rng = thread_rng();

    // Number of rooms correspond to map size
    let n = (self.w + self.h) / 10;

    for _ in 0..n {

      let x: usize = rng.gen_range(1, self.w - 2);
      let y: usize = rng.gen_range(1, self.h - 2);
      let l: usize = rng.gen_range(5, 20);
      let w: usize = rng.gen_range(5, 20);

      // Check bounds
      if w + x >= self.w || l + y >= self.h {
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

    let mut mover = (c.start.0, c.start.1);

    while mover.0 != c.end.0 {

      if mover.0 < c.end.0 {
        mover.0 += 1;
      } else if mover.0 > c.end.0 {
        mover.0 -= 1;
      } 

      self.grid.0[mover.0 as usize][mover.1 as usize] = Tile::new(
        "Floor".to_string(),
        ' ',
        (255, 255, 255), 
        (0, 0, 0), 
        false
      );

    }

    while mover.1 != c.end.1 {

      if mover.1 < c.end.1 {
        mover.1 += 1;
      } else if mover.1 > c.end.1 {
        mover.1 -= 1;
      } 

      self.grid.0[mover.0][mover.1] = Tile::new(
        "Floor".to_string(),
        ' ',
        (255, 255, 255), 
        (0, 0, 0), 
        false
      );

    }

  }

  /// 
  /// Build a rectangle to the grid
  /// 
  fn build_rect(&mut self, r: &Rect) {
    for w in 0..r.w {
      for l in 0..r.l {
        self.grid.0[(w + r.x)][(l + r.y)] = Tile::new(
          "Floor".to_string(),
          ' ',
          (255, 255, 255), 
          (0, 0, 0), 
          false
        );
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

      let c1 : (usize, usize);
      let c2 : (usize, usize);

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
  /// Return a new `Dungeon`
  /// 
  pub fn new(grid: Grid<Tile>) -> Simple {

    // Make a new dungeon with our fresh grid of size `w` by `h`
    let s = Simple { 
      grid: grid.clone(), 
      rooms: Vec::<Rect>::new(), 
      w: grid.0.len(), 
      h: grid.0[0].len() 
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