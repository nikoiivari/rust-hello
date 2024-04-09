extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
//use sdl2::video::GLProfile;

use gl::types::{GLuint, GLint, GLchar};

use std::time::Duration;
use std::ffi::{CString, CStr};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3); //OpenGL 3.3.0

    let mut winbuild = sdl2::video::WindowBuilder::new(&video_subsystem,"Test", 800, 600);
    winbuild.resizable();
    let window = winbuild.opengl().build().unwrap();
    
    let _context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let vs = build_vertex_shader();
    let fs = build_fragment_shader();
    let prog = link_shaders(vs, fs);

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
            //println!("VertexArrayId: {vertexarrayid}");

            let mut vertexbuffer: GLuint = 0;
            gl::GenBuffers(1, &mut vertexbuffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertexbuffer);
            //println!("VertexBufferId: {vertexbuffer}");
            
            gl::EnableVertexAttribArray(0); //index
            gl::VertexAttribPointer(
                                    0, //index
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    0,
                                    std::ptr::null()
            );
            
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, //size in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, //pointer
                gl::STATIC_DRAW,
            );

            gl::UseProgram(prog);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DisableVertexAttribArray(0);

            //release stuff
            gl::DeleteVertexArrays(1, &mut vertexarrayid);
            gl::DeleteBuffers(1, &mut vertexbuffer);
        }

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); //60
    }

}

fn build_vertex_shader() -> GLuint {
    let vshader:GLuint = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };

    let source: &CStr  = CStr::from_bytes_with_nul(b"#version 330 core
    layout(location = 0) in vec3 vertexPos;
    void main (){
        gl_Position.xyz = vertexPos;
        gl_Position.w = 1.0;
    }\0").unwrap();

    unsafe {
        gl::ShaderSource(vshader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(vshader);
        let mut success: GLint = 1;
        gl::GetShaderiv(vshader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len:GLint = 0;
            gl::GetShaderiv(vshader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf: Vec<u8> = Vec::with_capacity(len as usize +1);
            buf.extend([b' '].iter().cycle().take(len as usize));
            let error: CString = CString::from_vec_unchecked(buf);
            gl::GetShaderInfoLog(
                vshader,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar
            );
            println!("{}", error.to_string_lossy().into_owned());
        }
    }

    return vshader;
}

fn build_fragment_shader() -> GLuint {
    let fshader:GLuint = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

    let source: &CStr  = CStr::from_bytes_with_nul(b"#version 330 core
    out vec4 fragColor;
    void main (){
        fragColor = vec4(1.0, 0.0, 0.0, 1.0);
    }\0").unwrap();

    unsafe {
        gl::ShaderSource(fshader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(fshader);
        let mut success: GLint = 1;
        gl::GetShaderiv(fshader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len:GLint = 0;
            gl::GetShaderiv(fshader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf: Vec<u8> = Vec::with_capacity(len as usize +1);
            buf.extend([b' '].iter().cycle().take(len as usize));
            let error: CString = CString::from_vec_unchecked(buf);
            gl::GetShaderInfoLog(
                fshader,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar
            );
            println!("{}", error.to_string_lossy().into_owned());
        }
    }
    return fshader;
}

fn link_shaders(vs:GLuint, fs:GLuint) -> GLuint {
    let prog:GLuint = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(prog, vs);
        gl::AttachShader(prog, fs);
        gl::LinkProgram(prog);
    }
    return prog;
}