use nalgebra::{Point2, Vector2};
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

    let (tile_size_x, tile_size_y) = (window_width / 8, window_height / 8);

    let mut renderer = window.into_canvas().accelerated().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player_rect = Rect::new((window_width / 2) as i32, (window_height / 2) as i32, 25, 25); // slap playa in the center of the screen
    let player_movement_speed: i32 = 15; // player movement speed

    let mut player_angle: f64 = 0.0; // player angle (in rads)
    let player_rotation_speed: f64 = 0.1; // player turning speed, in rads/frame

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

        // Player movement logic
        let keys_pressed = KeyboardState::new(&event_pump);

        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::W) { // moves player up
            player_rect.y -= player_movement_speed;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::A) { // moves player left
            player_rect.x -= player_movement_speed;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::S) { // moves player down
            player_rect.y += player_movement_speed;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::D) { // moves player right
            player_rect.x += player_movement_speed;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::Left) { // turns player left
            player_angle -= player_rotation_speed;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::Right) { // turns player right
            player_angle += player_rotation_speed;
        }

        renderer.set_draw_color(Color::RGB(110, 110, 110));
        renderer.clear();

        utils::draw_map(&mut renderer, &MAP, tile_size_x, tile_size_y, window_width, window_height);

        // stupid fucked up math shit (basically calculating the endpoint of the ray)
        let ray_length = 600; // this is the length of the ray in px
        let player_center = player_rect.center();
        let ray_end_x = (player_center.x as f64 + player_angle.cos() * ray_length as f64) as i32;
        let ray_end_y = (player_center.y as f64 + player_angle.sin() * ray_length as f64) as i32;

        // draws the ray TODO! factor this shit out
        renderer.set_draw_color(Color::RGB(250, 0, 105));
        renderer.draw_line(player_center, (ray_end_x, ray_end_y)).unwrap();

        renderer.set_draw_color(Color::RGB(240, 100, 160));
        renderer.fill_rect(player_rect).unwrap();

        renderer.present();

        sleep(Duration::from_millis(33));
    }
}

