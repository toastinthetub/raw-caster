// utils.rs

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

pub fn draw_map(canvas: &mut Canvas<sdl2::video::Window>, map: &[u32; 64], tile_size: u32, window_width: u32, window_height: u32) {
    let num_cols = window_width / tile_size;
    let num_rows = window_height / tile_size;

    for row in 0..8 {
        for col in 0..8 {
            let index = row * 8 + col;
            let x = col * tile_size;
            let y = row * tile_size;

            let color = match map[index as usize] {
                1 => Color::RGB(255, 255, 255), // Wall
                0 => Color::RGB(0, 0, 0),       // Path
                _ => panic!("Invalid tile value"),
            };

            let rect = Rect::new(x as i32, y as i32, tile_size - 1, tile_size - 1);
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();
        }
    }
}
