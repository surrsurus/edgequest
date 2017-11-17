// http://staffwww.itn.liu.se/~stegu/aqsis/aqsis-newnoise/

use game::dungeon::rand::{thread_rng, Rng};

pub struct Simplex {
  grad3: Vec<Vec<i8>>,
  perm: Vec<u8>,
}

impl Simplex {

  pub fn new() -> Simplex {
    return Simplex { 
      perm: Vec::new(), 
      grad3: vec![
        vec![1, 1, 0], vec![-1, 1, 0], vec![1, -1, 0], vec![-1, -1, 0],  
        vec![1, 0, 1], vec![-1, 0, 1], vec![1, 0, -1], vec![-1, 0, -1],  
        vec![0, 1, 1], vec![0, -1, 1], vec![0, 1, -1], vec![0, -1, -1]
      ]
    };
  }

  pub fn seed(&mut self) {
    let mut rng = thread_rng();
    let p : Vec<u8> = rng.gen_iter::<u8>().take(256).collect::<Vec<u8>>();
    let mut perm = Vec::<u8>::new();
    for i in 0..512 {
      perm.push(p[(i & 255) as usize]);
    }
    self.perm = perm;
  }

  fn dot2(&self, g: &Vec<i8>, x: f32, y: f32) -> f32 {
    g[0 as usize] as f32 * x + g[1 as usize] as f32 * y
  }

  fn dot3(&self, g: &Vec<i8>, x: f32, y: f32, z: f32) -> f32 {
    g[0 as usize] as f32 * x + g[1 as usize] as f32 * y + g[2 as usize] as f32 * z
  }

  pub fn noise_2d(&self, xin: f32, yin: f32) -> f32 {
    let mut n0 : f32;
    let mut n1 : f32;
    let mut n2 : f32;
    let F2 = 0.366025403;
    let s = (xin + yin) * F2;
    let i = (xin + s).floor() as i32;
    let j = (yin + s).floor() as i32;
    let G2 = 0.211324865;
    let t = (i + j) as f32 * G2;
    let X0 = i as f32 -t; // Unskew the cell origin back to (x,y) space 
    let Y0 = j as f32 -t; 
    let x0 = xin-X0; // The x,y distances from the cell origin 
    let y0 = yin-Y0;
    let i1 : f32;
    let j1 : f32; // Offsets for second (middle) corner of simplex in (i,j) coords 
    if x0>y0 {
      i1 = 1.0; j1 = 0.0;
    } // lower triangle, XY order: (0,0)->(1,0)->(1,1) 
    else {
      i1=0.0; 
      j1=1.0;
    }
    let x1 = x0 - i1 + G2; // Offsets for middle corner in (x,y) unskewed coords 
    let y1 = y0 - j1 + G2; 
    let x2 = x0 - 1.0 + 2.0 * G2; // Offsets for last corner in (x,y) unskewed coords 
    let y2 = y0 - 1.0 + 2.0 * G2; 
    let ii = (i & 255) as f32; 
    let jj = (j & 255) as f32; 
    let gi0 = self.perm[(ii as i32 + self.perm[jj as usize] as i32) as usize ] % 12; 
    let gi1 = self.perm[(ii as i32 + i1 as i32 +self.perm[(jj + j1) as usize] as i32) as usize ] % 12; 
    let gi2 = self.perm[(ii as i32 + 1 + self.perm[ (jj + 1.0) as usize] as i32) as usize] % 12; 
    
    let mut t0 = 0.5 - x0*x0-y0*y0; 
    if t0 < 0.0 {
      n0 = 0.0;
    } 
    else { 
      t0 *= t0; 
      n0 = t0 * t0 * self.dot2(&self.grad3[gi0 as usize], x0, y0);  // (x,y) of grad3 used for 2D gradient 
    } 
    let mut t1 = 0.5 - x1*x1-y1*y1; 
    if t1 < 0. { 
      n1 = 0.0;
    } else { 
      t1 *= t1; 
      n1 = t1 * t1 * self.dot2(&self.grad3[gi1 as usize], x1, y1); 
    }
    let mut t2 = 0.5 - x2*x2-y2*y2; 
    if t2 < 0.0 { 
      n2 = 0.0; 
    }
    else { 
      t2 *= t2; 
      n2 = t2 * t2 * self.dot2(&self.grad3[gi2 as usize], x2, y2); 
    } 

    // Add contributions from each corner to get the final noise value. 
    // The result is scaled to return values in the interval [-1,1]. 

    return 70.0 * (n0 + n1 + n2);

  }

}