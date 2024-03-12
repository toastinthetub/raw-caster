use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub fn draw_map(MAP: [u32; 64]) {

    let side_length = 8;

    for i in 0..MAP.len() {
        print!(" {} ", MAP[i]);

        if (i + 1) % side_length == 0 {
            print!("\n");
        }
    }
}
