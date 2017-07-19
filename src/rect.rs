use pos::Pos;
use std::cmp::max;
use std::cmp::min;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
   pub min_x: usize,
   pub min_y: usize,
   pub max_x: usize,
   pub max_y: usize
}

impl Rect {
   pub fn new(x1: usize, x2: usize, y1: usize, y2: usize) -> Rect {
      let min_x = min(x1, x2);
      let max_x = max(x1, x2);
      let min_y = min(x1, x2);
      let max_y = max(x1, x2);
      Rect { min_x, max_x, min_y, max_y }
   }

   pub fn contains(self, p: Pos) -> bool {
      self.min_x <= p.x && p.x <= self.max_x
         && self.min_y <= p.y && p.y <= self.max_y
   }
}

impl IntoIterator for Rect {
   type Item = (Pos);
   type IntoIter = RectIterator;
   fn into_iter(self) -> Self::IntoIter {
      RectIterator::new(self)
   }
}


impl fmt::Display for Rect {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "(x: {} to {}, y: {} to {})", self.min_x, self.max_x, self.min_y, self.max_y)
   }
}

pub struct RectIterator {
   i: Pos,
   r: Rect
}

impl RectIterator {
   pub fn new(r: Rect) -> RectIterator {
      RectIterator { i: Pos { x: r.min_x, y: r.min_y }, r }
   }
}


impl Iterator for RectIterator {
   type Item = Pos;
   fn next(&mut self) -> Option<Pos> {
      if self.i.x <= self.r.max_x {
         let t = Some(self.i);
         self.i.x += 1;
         return t;
      }
      if self.i.y < self.r.max_y {
         self.i.y += 1;
         self.i.x = self.r.min_x;
         let t = Some(self.i);
         self.i.x += 1;
         return t;
      }
      None
   }
}