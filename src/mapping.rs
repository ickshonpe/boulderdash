use std::ops::Index;
use std::ops::IndexMut;
use rect::*;
use pos::*;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
   Player,
   Wall,
   Boulder,
   Monster,
   Diamond,
   Exit,
   Mud,
   Empty
}

impl Default for Tile {
   fn default() -> Self {
      Tile::Empty
   }
}

#[derive(Copy, Clone)]
pub struct MapCell {
   pub tile: Tile,
   pub cool_down: i64,
   pub facing: Option<Direction>
}

impl Default for MapCell {
   fn default() -> Self {
      Self {
         tile: Tile::default(),
         cool_down: 0,
         facing: None
      }
   }
}

impl MapCell {
   pub fn is_cool(&self) -> bool {
      self.cool_down <= 0
   }

   pub fn is_empty(&self) -> bool {
      self.tile == Tile::Empty
   }
}

#[derive(Clone)]
pub struct LevelMap {
   data: Vec<Vec<MapCell>>
}




impl LevelMap {
   pub fn new(x_len: usize, y_len: usize) -> LevelMap {
      let data = make_2d_vec(x_len, y_len, MapCell::default());
      LevelMap { data: data }
   }
   pub fn x_max(&self) -> usize { self.x_len() - 1 }
   pub fn y_max(&self) -> usize { self.y_len() - 1 }
   pub fn x_len(&self) -> usize {
      self.data.len()
   }
   pub fn y_len(&self) -> usize {
      self.data[0].len()
   }
   pub fn get(&self, x: usize, y: usize) -> MapCell {
      self.data[x][y]
   }
   pub fn iter_points(&self) -> RectIterator {
      self.rect().into_iter()
   }
   pub fn rect(&self) -> Rect {
      Rect { min_x: 0, min_y: 0, max_x: self.x_len() - 1, max_y: self.y_len() - 1 }
   }
   pub fn has_no_diamonds(&self) -> bool {
      for (i, c) in self {
         if c.tile == Tile::Diamond {
            return false;
         }
      }
      true
   }
}

pub struct LevelIntoIterator {
   i: Pos,
   m: LevelMap
}

pub struct LevelIterator<'a> {
   i: Pos,
   m: &'a LevelMap
}

impl<'a> Iterator for LevelIterator<'a> {
   type Item = (Pos, MapCell);
   fn next(&mut self) -> Option<Self::Item> {
      if self.i.x < self.m.x_len() {
         let r = Some((self.i, self.m[self.i]));
         self.i.x += 1;
         r
      } else {
         self.i.y += 1;
         self.i.x = 0;
         if self.i.y < self.m.y_len() {
            let r = Some((self.i, self.m[self.i]));
            self.i.x += 1;
            r
         } else {
            None
         }
      }
   }
}

impl<'a> IntoIterator for &'a LevelMap {
   type Item = (Pos, MapCell);
   type IntoIter = LevelIterator<'a>;
   fn into_iter(self) -> Self::IntoIter {
      LevelIterator { i: Pos { x: 0, y: 0 }, m: self }
   }
}


impl IntoIterator for LevelMap {
   type Item = (Pos, MapCell);
   type IntoIter = LevelIntoIterator;
   fn into_iter(self) -> Self::IntoIter {
      LevelIntoIterator { i: Pos { x: 0, y: 0 }, m: self }
   }
}


impl Iterator for LevelIntoIterator {
   type Item = (Pos, MapCell);
   fn next(&mut self) -> Option<(Pos, MapCell)> {
      if self.i.x < self.m.x_len() {
         let r = Some((self.i, self.m[self.i]));
         self.i.x += 1;
         r
      } else {
         self.i.y += 1;
         self.i.x = 0;
         if self.i.y < self.m.y_len() {
            let r = Some((self.i, self.m[self.i]));
            self.i.x += 1;
            r
         } else {
            None
         }
      }
   }
}

impl Index<Pos> for LevelMap {
   type Output = MapCell;
   fn index(&self, p: Pos) -> &MapCell {
      &self.data[p.x][p.y]
   }
}

impl IndexMut<Pos> for LevelMap {
   fn index_mut(&mut self, index: Pos) -> &mut MapCell {
      &mut self.data[index.x][index.y]
   }
}

fn make_2d_vec<T>(x_len: usize, y_len: usize, default_value: T) -> Vec<Vec<T>>
   where T: Clone {
   vec![vec![default_value; y_len]; x_len]
}

pub fn print_map(m: &LevelMap) {
   for (p, c) in m {
      print_glyph(c.tile);
      if p.x + 1 == m.x_len() {
         println!();
      }
   }
}

pub fn print_glyph(t: Tile) {
   let c = match t {
      Tile::Player => "@",
      Tile::Wall => "#",
      Tile::Boulder => "O",
      Tile::Diamond => "*",
      Tile::Exit => "X",
      Tile::Monster => "H",
      Tile::Mud => "m",
      Tile::Empty => "."
   };
   print!("{}", c);
}

impl LevelMap {
   pub fn find_player(&self) -> Option<Pos> {
      for (p, t) in self {
         if t.tile == Tile::Player {
            return Some(p);
         }
      }
      None
   }
}