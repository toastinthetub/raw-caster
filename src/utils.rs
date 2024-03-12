use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

pub fn draw_map(canvas: &mut Canvas<sdl2::video::Window>, map: &[u32; 64], tile_size_x: u32, tile_size_y: u32, window_width: u32, window_height: u32) {
    let num_cols = window_width / tile_size_x;
    let num_rows = window_height / tile_size_y;

    for row in 0..num_rows {
        for col in 0..num_cols {
            let index = (row * num_cols + col) as usize;
            let x = col * tile_size_x;
            let y = row * tile_size_y;

            let color = match map[index] {
                1 => Color::RGB(0, 0, 0), // Wall
                0 => Color::RGB(120, 220, 240),       // Path
                _ => panic!("Invalid tile value"),
            };

            let rect = Rect::new(x as i32, y as i32, tile_size_x - 1, tile_size_y - 1);
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();
        }
    }
}