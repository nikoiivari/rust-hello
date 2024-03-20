extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
//use sdl2::video::GLProfile;

use gl::types::GLuint;

use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut winbuild = sdl2::video::WindowBuilder::new(&video_subsystem,"Test", 800, 600);
    winbuild.resizable();
    let window = winbuild.opengl().build().unwrap();

    let _context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let mut event_pump = sdl_context.event_pump().unwrap();

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
        
        unsafe {
            gl::ClearColor(0.0, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let vertices: Vec<f32> = vec![0.0, 0.0, 0.0,
                                          0.0, 1.0, 0.0,
                                          1.0, 1.0, 0.0];
                                          
            let mut vertexarrayid: GLuint = 0;
            gl::GenVertexArrays(1, &mut vertexarrayid);
            gl::BindVertexArray(vertexarrayid);
            println!("VertexArrayId: {vertexarrayid}");

            let mut vertexbuffer: GLuint = 0;
            gl::GenBuffers(1, &mut vertexbuffer);
            //gl::BindBuffer(GL_ARRAY_BUFFER, vertexbuffer);
            println!("VertexBufferId: {vertexbuffer}");


            //release stuff
            gl::DeleteVertexArrays(1, &mut vertexarrayid);
            gl::DeleteBuffers(1, &mut vertexbuffer);
        }

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 6));
    }


}
