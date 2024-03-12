use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode};
use std::time::Duration;
use std::thread::sleep;

use utils::draw_map;

mod utils;

const MAP: [u32; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 1, 0, 0, 0, 0, 1,
    1, 0, 1, 0, 0, 0, 0, 1,
    1, 0, 1, 1, 1, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1,
];

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();


    let window = video_subsystem.window("SDL Example", 800, 600)
        .position_centered()
        .build()
        .unwrap();


    let mut renderer = window.into_canvas()
        .accelerated()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump();
    
  
    let mut rect_x = 100;
    let mut rect_y = 100;
    let rect_width = 25; 
    let rect_height = 25; 

    let user_keyboard = KeyboardState::new(&event_pump.as_mut().unwrap());

    'mainloop: loop {

        for event in event_pump.as_mut().unwrap().poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop;
                },
                _ => {}
            }
        }

        let keys_pressed = match event_pump.as_ref() {
            Ok(event_pump) => KeyboardState::new(event_pump),
            Err(err) => {
                eprintln!("Error: {}", err);
                continue 'mainloop;
            }
        };

        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            rect_y -= 25;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            rect_x -= 25;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            rect_y += 25;
        }
        if keys_pressed.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            rect_x += 25;
        }
        
        draw_map(MAP);

        renderer.set_draw_color(Color::RGB(110, 110, 110));
        renderer.clear();

        renderer.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(rect_x, rect_y, rect_width, rect_height);
        renderer.fill_rect(rect).unwrap();

        renderer.present();

        sleep(Duration::from_millis(33));

    }
}