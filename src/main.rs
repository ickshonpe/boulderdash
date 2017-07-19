mod mapping;
mod game;
mod rect;
mod pos;
mod util;
mod mapgen;
mod assets;

extern crate rand;
extern crate vecmath;
extern crate piston_window;
extern crate find_folder;

use std::fs::File;
use std::path::Path;
use mapping::*;
use game::*;
use rect::Rect;
use pos::*;
use vecmath::*;
use piston_window::*;
use image::*;
use mapgen::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
   let mut old_player_pos = Pos { x: 1, y: 1 };
   let map_data = vec![level1(), level2(), level3()];
   let mut level = 0;
   let texture_files = vec![
      (Tile::Empty, "empty.png"),
      (Tile::Player, "man.png"),
      (Tile::Mud, "mud.png"),
      (Tile::Wall, "wall.png"),
      (Tile::Boulder, "boulder.png"),
      (Tile::Diamond, "diamond.png"),
      (Tile::Monster, "butterfly.png"),
      (Tile::Exit, "exit.png")];
   let mut window: PistonWindow =
      WindowSettings::new("Boulderdash", [1280, 720])
         .fullscreen(false)
         .vsync(true)
         .exit_on_esc(true)
         .build()
         .unwrap();
   let mut input_state = HashSet::<Direction>::new();
   let mut old_input_state = HashSet::<Direction>::new();
   let mut textures = std::collections::HashMap::new();
   for (t, f) in texture_files {
      let path = assets::find_asset(f);
      let new_texture =
         Texture::from_path(
            &mut window.factory,
            path,
            Flip::Vertical,
            &TextureSettings::new()).unwrap();
      textures.insert(t, new_texture);
   }
   let factory = window.factory.clone();
   let font_path = assets::find_asset("font.ttf");
   let mut glyphs = Glyphs::new(font_path, factory).unwrap();
   let mut game_state = GameState::new(read_map(&map_data[level]), 0, 3);
   while let Some(e) = window.next() {
      if let Input::Update(_) = e {
         update_game(&mut game_state);
         if game_state.level_complete {
            level += 1;
            if level == map_data.len() {
               level = 0;
            }
            game_state = GameState::new(read_map(&map_data[level]), game_state.score, game_state.player_lives);
         } else if !game_state.player_alive {
            game_state.player_lives = game_state.player_lives - 1;
            if game_state.player_lives == 0 {
               level = 0;
               game_state = GameState::new(read_map(&map_data[level]), 0, 3);
            } else {
               game_state = GameState::new(read_map(&map_data[level]), game_state.score, game_state.player_lives);
            }
         }
      }
      if let Input::Render(r) = e {
         window.draw_2d(&e, |context: Context, g2d| {
            let (cam_x, cam_y) =
            if let Some(player_pos) = game_state.map.find_player() {
               old_player_pos = player_pos;
                  util::calculate_camera_position(
                     (game_state.map.x_len() as f64 * 16.0, game_state.map.y_len() as f64 * 16.0),
                     (context.viewport.unwrap().window_size[0] as f64, context.viewport.unwrap().window_size[1] as f64),
                     (player_pos.x as f64 * 16.0 + 8.0, player_pos.y as f64 * 16.0 + 8.0)
                  )
            } else {
                  util::calculate_camera_position(
                     (game_state.map.x_len() as f64 * 16.0, game_state.map.y_len() as f64 * 16.0),
                     (context.viewport.unwrap().window_size[0] as f64, context.viewport.unwrap().window_size[1] as f64),
                     (old_player_pos.x as f64 * 16.0 + 8.0, old_player_pos.y as f64 * 16.0 + 8.0))
            };
            let height = context.viewport.unwrap().window_size[1] as f64;
            let context = context.trans(0.0, height);
            let context = context.scale(1.0, -1.0);
            let context = context.trans(cam_x, cam_y);
            let game_context = context.scale(1.0, 1.0);
            clear([0.0, 0.0, 0.0, 1.0], g2d);

            draw_map(&r, g2d, &game_state, game_context, &textures, (16.0, 16.0));
         });
         window.draw_2d(&e, |context, g2d| {
            let text_color = [0.0, 1.0, 1.0, 1.0];
            let text_context = context.trans(context.viewport.unwrap().window_size[0] as f64 - 300.0, 30.0);
            let score_string = std::fmt::format(format_args!("SCORE: {} ", game_state.score));
            text::Text::new_color(text_color, 32).draw(
               &score_string,
               &mut glyphs,
               &text_context.draw_state,
               text_context.transform,
               g2d);
            let text_color = [0.0, 1.0, 1.0, 1.0];
            let text_context = context.trans(context.viewport.unwrap().window_size[0] as f64 - 500.0, 30.0);
            let lives_string = std::fmt::format(format_args!("LIVES: {} ", game_state.player_lives));
            text::Text::new_color(text_color, 32).draw(
               &lives_string,
               &mut glyphs,
               &text_context.draw_state,
               text_context.transform,
               g2d);
         });
      }
      if let Input::Press(b) = e {
         if let Some(direction) = translate_input(b) {
            input_state.insert(direction);
            game_state.player_command = Some(direction);
         }
      }
      if let Input::Release(b) = e {
         if let Some(direction) = translate_input(b) {
            input_state.remove(&direction);
            game_state.player_command = None;
            let ds = vec![pos::Direction::Up, Direction::Left, Direction::Right, Direction::Down];
            for d in ds {
               if input_state.contains(&d) {
                  game_state.player_command = Some(d);
               }
            }
         }
      }
      if let Input::Close(_) = e {
         window.set_should_close(true);
      }
   }
}

fn draw_map(r: &RenderArgs, g2d: &mut G2d, game_state:
&GameState, context: Context, textures: &std::collections::HashMap<Tile, G2dTexture>, tile_size: (f64, f64)) {
   let m = &game_state.map;
   for (i, c) in m {
      let x = (i.x as f64) * tile_size.0;
      let y = (i.y as f64) * tile_size.1;
      let transform = context.trans(x, y).transform;
      let mut tile = c.tile;
      if c.tile == Tile::Exit && !m.has_no_diamonds() {
         tile = Tile::Wall;
      }
      let texture = textures.get(&tile).unwrap();
      image(texture, transform, g2d);
   }
}

fn translate_input(b: Button) -> Option<Direction> {
   let key_map: std::collections::HashMap<Key, Direction> = [
      (Key::W, Direction::Up),
      (Key::S, Direction::Down),
      (Key::A, Direction::Left),
      (Key::D, Direction::Right)].iter().cloned().collect();

   match b {
      Button::Keyboard(k) => {
         key_map.get(&k).cloned()
      }
      _ => {
         None
      }
   }
}
