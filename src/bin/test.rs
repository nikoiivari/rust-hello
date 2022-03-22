extern crate sdl2;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut winbuild = sdl2::video::WindowBuilder::new(&video_subsystem,"Test", 800, 600);
    winbuild.resizable();
    let window = winbuild.build().unwrap();

    //let window = video_subsystem.window("Hello!", 800, 600).build().unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 127, 127));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    //let mut win_width = 800;
    //let mut win_height = 600;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::Window {timestamp: _, window_id: _, win_event: wev} => {
                    match wev {
                        WindowEvent::SizeChanged(winw, winh) => {
                             println!("Size changed {}, {}", winw, winh);
                            //win_width = winw;
                            //win_height = winh;
                        }
                        _ => {}    
                    }
                    
                },
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    println!("Space");
                },
                //Event::MouseButtonDown {} => 
                Event::MouseButtonUp {mouse_btn:mouseb, x:mousex, y:mousey, .. } => {
                    if MouseButton::Left == mouseb {
                        println!("Mouse x:{}, y:{}", mousex, mousey);
                    }
                }
                _ => {}
            }
        }
        
        //canvas.set_draw_color(Color::RGB(0, 127, 127));
        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 480));
    }


}
