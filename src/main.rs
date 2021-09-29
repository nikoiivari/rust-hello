extern crate sdl2;

//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
//use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Hello!", 800, 600)
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    // Print text to the console
    println!("Hello World!");
}
