use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
   pub x: usize,
   pub y: usize
}

impl Pos {
   pub fn zero() -> Pos {
      Pos { x: 0, y: 0 }
   }
   pub fn with_x(&self, x: usize) -> Pos {
      Pos { x, y: self.y }
   }
   pub fn with_y(&self, y: usize) -> Pos {
      Pos { x: self.x, y }
   }
   pub fn up(&self) -> Pos {
      self.with_y(self.y + 1)
   }
   pub fn down(&self) -> Pos {
      self.with_y(self.y - 1)
   }
   pub fn left(&self) -> Pos {
      self.with_x(self.x - 1)
   }
   pub fn right(&self) -> Pos {
      self.with_x(self.x + 1)
   }
   pub fn to(&self, dir: Direction) -> Pos {
      match dir {
         Direction::Up => {
            return self.up();
         }
         Direction::Down => {
            return self.down();
         }
         Direction::Left => {
            return self.left();
         }
         Direction::Right => {
            return self.right();
         }
      }
   }
}

impl fmt::Display for Pos {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {})", self.x, self.y)
   }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
   Up,
   Down,
   Left,
   Right
}