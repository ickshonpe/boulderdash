extern crate rand;

use rand::Rng;
use pos::Direction;

pub fn select_random<T>(x: T, y: T) -> T {
   if rand::thread_rng().gen() { x } else { y }
}

pub fn select_random_direction() -> Direction {
   let ds = vec![Direction::Up, Direction::Down, Direction::Right, Direction::Left];
   *rand::thread_rng().choose(&ds).unwrap()
}

pub fn calculate_camera_position(map_size: (f64, f64), viewport_size: (f64, f64), player_position: (f64, f64)) -> (f64, f64) {
   let f = | g, c, p | if g < c { (c - g)/ 2.0 } else { p };
   let x = f(map_size.0, viewport_size.0, player_position.0);
   let y = f(map_size.1, viewport_size.1, player_position.1);
   (x, y)
}