use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::thread::sleep;
use std::time::Duration;

mod utils;

const PI: f64 = 3.1415926;

const MAP: [u32; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1, 
    1, 0, 1, 0, 0, 0, 0, 1, 
    1, 0, 1, 0, 0, 0, 0, 1,
    1, 0, 1, 1, 0, 0, 0, 1, 
    1, 0, 0, 0, 0, 0, 0, 1, 
    1, 0, 0, 0, 0, 1, 0, 1, 
    1, 0, 0, 0, 0, 1, 0, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 
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

    let (tile_size_x, tile_size_y) = (window_width / 8, window_height / 8);

    let mut renderer = window.into_canvas().accelerated().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player_rect = Rect::new((window_width / 2) as i32, (window_height / 2) as i32, 25, 25);
    let player_movement_speed: i32 = 15;

    let mut player_angle: f64 = 0.0; // starting angle (rads)
    let player_rotation_speed: f64 = 0.1; // rotation spd (rads/frame)

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
            player_rect.y -= player_movement_speed;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            player_rect.x -= player_movement_speed;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            player_rect.y += player_movement_speed;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            player_rect.x += player_movement_speed;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
            player_angle -= player_rotation_speed;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
            player_angle += player_rotation_speed;
        }

        renderer.set_draw_color(Color::RGB(110, 110, 110));
        renderer.clear();

        utils::draw_map(&mut renderer, &MAP, tile_size_x, tile_size_y, window_width, window_height);

        utils::cast_ray(&mut renderer, &player_rect, player_angle, 600); // Adjust ray length as needed

        renderer.set_draw_color(Color::RGB(240, 100, 160));
        renderer.fill_rect(player_rect).unwrap();

        renderer.present();

        sleep(Duration::from_millis(33));
    }
}

