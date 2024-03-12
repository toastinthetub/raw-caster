// main.rs

use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread::sleep;
use std::time::Duration;

mod utils;

const MAP: [u32; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1, //
    1, 0, 1, 0, 0, 0, 0, 1, //
    1, 0, 1, 0, 0, 0, 0, 1, //
    1, 0, 1, 1, 0, 0, 0, 1, //
    1, 0, 0, 0, 0, 0, 0, 1, //
    1, 0, 0, 0, 0, 1, 0, 1, //
    1, 0, 0, 0, 0, 1, 0, 1, //
    1, 1, 1, 1, 1, 1, 1, 1, //
];

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 1128;
    let window_height = 1504;

    let window = video_subsystem
        .window("SDL Example", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let (window_width, window_height) = window.size();

    let tile_size = (window_width / 8);

    let mut renderer = window.into_canvas().accelerated().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player_rect = Rect::new(100, 100, 15, 15);

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }

        let keys_pressed = KeyboardState::new(&event_pump);

        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            player_rect.y -= 25;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            player_rect.x -= 25;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            player_rect.y += 25;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            player_rect.x += 25;
        }

        renderer.set_draw_color(Color::RGB(110, 110, 110));
        renderer.clear();

        utils::draw_map(&mut renderer, &MAP, tile_size, window_width, window_height);

        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.fill_rect(player_rect).unwrap();

        renderer.present();

        sleep(Duration::from_millis(33));
    }
}
