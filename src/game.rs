use mapping::*;
use pos::*;
use std::collections::HashSet;
use util::*;

pub struct GameState {
   pub map: LevelMap,
   pub player_alive: bool,
   pub player_command: Option<Direction>,
   pub turn: i64,
   pub score: i64,
   pub crush_resistance: i64,
   pub crush: i64,
   pub level_complete: bool,
   pub player_lives: i64
}

impl GameState {
   pub fn new(level_map: LevelMap, score: i64, lives: i64) -> GameState {
      GameState {
         map: level_map,
         player_command: None,
         player_alive: true,
         turn: 0,
         score: score,
         crush_resistance: 30,
         crush: 30,
         level_complete: false,
         player_lives: lives
      }
   }
}

pub fn update_game(g: &mut GameState) {
   g.turn += 1;
   let mut touched: HashSet<Pos> = HashSet::new();
   let mut m = &mut g.map;
   let player_command = g.player_command;
   for p in m.iter_points() {
      m[p].cool_down -= 1;
      if !touched.contains(&p) {
         touched.insert(p);
         let c: MapCell = m[p];
         if c.is_cool() {
            match c.tile {
               Tile::Boulder => {
                  let r = p.down();
                  if m[r].tile == Tile::Player {
                     if g.crush == 0 {
                        g.player_alive = false;
                        m[p].tile = Tile::Empty;
                        m[r].tile = Tile::Boulder;
                        m[r].cool_down = 20;
                        touched.insert(r);
                     } else {
                        g.crush -= 1;
                     }
                  } else if m[r].is_empty() {
                     m[p].tile = Tile::Empty;
                     m[r].tile = Tile::Boulder;
                     m[r].cool_down = 20;
                     touched.insert(r);
                  } else {
                     let d = select_random(Direction::Left, Direction::Right);
                     let n = p.to(d);
                     let r = r.to(d);
                     if m[r].is_empty() && m[n].is_empty() {
                        m[p].tile = Tile::Empty;
                        m[r].tile = Tile::Boulder;
                        m[r].cool_down = 20;
                        touched.insert(r);
                     } else if m[r].tile == Tile::Player && m[n].is_empty() {
                        if g.crush == 0 {
                           g.player_alive = false;
                           m[p].tile = Tile::Empty;
                           m[r].tile = Tile::Boulder;
                           m[r].cool_down = 20;
                           touched.insert(r);
                        } else {
                           g.crush -= 1;
                        }
                     }
                  }
               }
               Tile::Monster => {
                  if let Some(dir) = m[p].facing {
                     let r = p.to(dir);
                     match m[r].tile {
                        Tile::Empty => {
                           m[p].tile = Tile::Empty;
                           m[r].tile = Tile::Monster;
                           m[r].facing = m[p].facing;
                           m[r].cool_down = 7;
                           touched.insert(r);
                        }
                        Tile::Player => {
                           m[p].tile = Tile::Empty;
                           m[r].tile = Tile::Monster;
                           m[r].facing = m[p].facing;
                           m[r].cool_down = 7;
                           touched.insert(r);
                           g.player_alive = false;
                        }
                        _ => {
                           m[p].facing = Some(select_random_direction());
                           m[p].cool_down = 10;
                        }
                     }
                  } else {
                     m[p].facing = Some(select_random_direction());
                  }
               }
               Tile::Player => {
                  match player_command {
                     Some(command) => {
                        let r = p.to(command);
                        match m[r].tile {
                           Tile::Empty | Tile::Mud => {
                              m[p].tile = Tile::Empty;
                              m[r].tile = Tile::Player;
                              m[r].cool_down = 10;
                              touched.insert(r);
                              g.crush = g.crush_resistance;
                           }
                           Tile::Diamond => {
                              m[p].tile = Tile::Empty;
                              m[r].tile = Tile::Player;
                              m[r].cool_down = 10;
                              touched.insert(r);
                              g.score += 1;
                              g.crush = g.crush_resistance;
                           }
                           Tile::Monster => {
                              m[p].tile = Tile::Empty;
                              g.player_alive = false;
                           }
                           Tile::Exit => {
                              if m.has_no_diamonds() {
                                 g.level_complete = true;
                              }
                           }
                           _ => {}
                        }

                     }
                     None => {}
                  }
               }
               _ => {}
            }
         }
      }
   }
}





