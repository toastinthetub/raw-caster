use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::sys::Window;

#[derive(Debug)]
pub struct Player {
    pub player_rect: Rect,
    pub player_movement_speed: i32,

    pub player_angle: f64,
    pub player_rotation_speed: f64,
}

impl Player {
    pub fn new(window: &sdl2::video::Window, player_movements_speed: &i32, player_angle: &f64, player_rotation_speed: &f64) -> Self {

        let (window_width, window_height) = window.size();

        Player {
            player_rect: Rect::new((window_width / 2) as i32, (window_height / 2) as i32, 25, 25),
            player_movement_speed: 15,

            player_angle: 0.0,
            player_rotation_speed: 0.1,
        }
    }
    pub fn w_key_pressed(&mut self) {
        self.player_rect.y -= self.player_movement_speed;
    }
    pub fn a_key_pressed(&mut self) {
        self.player_rect.x -= self.player_movement_speed;
    }
    pub fn s_key_pressed(&mut self) {
        self.player_rect.y += self.player_movement_speed;
    }
    pub fn d_key_pressed(&mut self) {
        self.player_rect.x += self.player_movement_speed;
    }
    pub fn left_key_pressed(&mut self) {
        self.player_angle -= self.player_rotation_speed;
    }
    pub fn right_key_pressed(&mut self) {
        self.player_angle += self.player_rotation_speed;
    }
}

pub fn draw_map(
    canvas: &mut Canvas<sdl2::video::Window>,
    map: &[u32; 64],
    tile_size_x: u32,
    tile_size_y: u32,
    window_width: u32,
    window_height: u32,
) {
    let num_cols = window_width / tile_size_x;
    let num_rows = window_height / tile_size_y;

    for row in 0..num_rows {
        for col in 0..num_cols {
            let index = (row * num_cols + col) as usize;
            let x = col * tile_size_x;
            let y = row * tile_size_y;

            let color = match map[index] {
                1 => Color::RGB(0, 0, 0), // Wall
                0 => Color::RGB(100, 100, 100), // Path
                _ => panic!("Invalid tile value"),
            };

            let rect = Rect::new(x as i32, y as i32, tile_size_x - 1, tile_size_y - 1);
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();
        }
    }
}

pub fn cast_ray(
    canvas: &mut Canvas<sdl2::video::Window>,
    player_rect: &Rect,
    player_angle: f64,
    ray_length: i32,
) {
    let player_center = player_rect.center();
    let ray_end_x = (player_center.x as f64 + player_angle.cos() * ray_length as f64) as i32;
    let ray_end_y = (player_center.y as f64 + player_angle.sin() * ray_length as f64) as i32;

    canvas.set_draw_color(Color::RGB(250, 0, 105));
    canvas.draw_line(player_center, Point::new(ray_end_x, ray_end_y)).unwrap();
}