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

    // Mouse rotation
    let mut rotatex:f32 = 0.0;
    let mut rotatey:f32 = 0.0;
    let mut rotate_ongoing:bool = false;

    let vs = build_vertex_shader();
    let fs0 = build_fragment_shader(0);
    let fs1 = build_fragment_shader(1);
    let prog0 = link_shaders(vs, fs0);
    let prog1 = link_shaders(vs, fs1);

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
                            unsafe { gl::Viewport(0, 0, winw, winh); }
                        }
                        _ => {}    
                    }
                    
                },
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    println!("Space");
                },
                Event::MouseButtonDown {mouse_btn:mouseb, x:_mousex, y:_mousey, .. } => {
                    if MouseButton::Middle == mouseb {                        
                        rotate_ongoing = true;
                    }
                }
                Event::MouseButtonUp {mouse_btn:mouseb, x:mousex, y:mousey, .. } => {
                    if MouseButton::Left == mouseb {
                        println!("Mouse x:{}, y:{}", mousex, mousey);
                    }
                    if MouseButton::Middle == mouseb {
                        rotate_ongoing = false;
                    }
                }
                Event::MouseMotion {xrel:mousex, yrel:mousey, .. } => {
                    if rotate_ongoing {
                        rotatey = rotatey + mousex as f32;
                        if rotatey > 360.0 { rotatey = rotatey - 360.0; }
                        if rotatey < 0.0 { rotatey = 360.0 - rotatey; }
                        //TODO: rotatex/mousey
                        rotatex = rotatex + mousey as f32;
                        if rotatex > 360.0 { rotatex = rotatex - 360.0; }
                        if rotatex < 0.0 { rotatex = 360.0 - rotatex;}
                    }
                }
                _ => {}
            }
        }
        
        unsafe {
            gl::ClearColor(0.0, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let vertices: Vec<f32> = vec![0.0, 0.0, 0.0,
                                          0.0, 0.0, -1.0,
                                          1.0, 0.0, -1.0,
                                          
                                          0.0, 0.0, 0.0,
                                          1.0, 0.0, -1.0,
                                          1.0, 0.0, 0.0,
                                          
                                          0.0, 0.0, 0.0,
                                          0.0, 0.0, 1.0,
                                          -1.0, 0.0, 1.0,
                                          
                                          0.0, 0.0, 0.0,
                                          -1.0, 0.0, 1.0,
                                          -1.0, 0.0, 0.0,]; //12
                                        
            let vertices2: Vec<f32> = vec![0.0, 0.0, 0.0,
                                          -1.0, 0.0, 0.0,
                                          -1.0, 0.0, -1.0,
                                          
                                          0.0, 0.0, 0.0,
                                          -1.0, 0.0, -1.0,
                                          0.0, 0.0, -1.0,
                                          
                                          0.0, 0.0, 0.0,
                                          0.0, 0.0, 1.0,
                                          1.0, 0.0, 1.0,
                                          
                                          0.0, 0.0, 0.0,
                                          1.0, 0.0, 1.0,
                                          1.0, 0.0, 0.0,]; //12

            // vertices
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

            gl::UseProgram(prog0);
            gl::Uniform2f(0, rotatex, rotatey);
            gl::Uniform4f(1, 0.4, 0.4, 0.4, 1.0);
            gl::DrawArrays(gl::TRIANGLES, 0, 12);
            gl::DisableVertexAttribArray(0);

            //release stuff
            gl::DeleteVertexArrays(1, &mut vertexarrayid);
            gl::DeleteBuffers(1, &mut vertexbuffer);

            //vertices2
            let mut vertexarrayid2: GLuint = 0;
            gl::GenVertexArrays(1, &mut vertexarrayid);
            gl::BindVertexArray(vertexarrayid);
            //println!("VertexArrayId: {vertexarrayid}");

            let mut vertexbuffer2: GLuint = 0;
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
                (vertices2.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, //size in bytes
                vertices2.as_ptr() as *const gl::types::GLvoid, //pointer
                gl::STATIC_DRAW,
            );

            gl::UseProgram(prog1);
            gl::Uniform2f(0, rotatex, rotatey);
            gl::Uniform4f(1, 0.9, 0.9, 0.9, 1.0);
            gl::DrawArrays(gl::TRIANGLES, 0, 12);
            gl::DisableVertexAttribArray(0);
            
            //release more stuff
            gl::DeleteVertexArrays(1, &mut vertexarrayid2);
            gl::DeleteBuffers(1, &mut vertexbuffer2);
        }

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); //60
    }

}

fn build_vertex_shader() -> GLuint {
    let vshader:GLuint = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };

    let source: &CStr  = CStr::from_bytes_with_nul(b"#version 330 core
    #define PI 3.1415926538
    layout(location = 0) in vec3 vertexPos;

    uniform vec2 rotate;

    mat4 tmZ(float translationZ)
    {
        mat4 tm = mat4(0.0);
        tm[0][0]=1.0;
        tm[1][1]=1.0;

        tm[2][2]= 1.0;

        tm[2][3]=translationZ;
        tm[3][3]=1.0;
        return tm;
    }

    mat4 tmV(vec3 V)
    {
        mat4 tm = mat4(0.0);
        tm[0][0]=1.0;   tm[1][0]=0.0;   tm[2][0]=0.0;   tm[3][0]=V.x;
        tm[0][1]=0.0;   tm[1][1]=1.0;   tm[2][1]=0.0;   tm[3][1]=V.y;
        tm[0][2]=0.0;   tm[1][2]=0.0;   tm[2][2]=1.0;   tm[3][2]=V.z;
        tm[0][3]=0.0;   tm[1][3]=0.0;   tm[2][3]=0.0;   tm[3][3]=1.0;
        return tm;
    }

    mat4 rmX(float rotationX)
    {
        float rx = rotationX/180.0 * PI;
        mat4 rm = mat4(0.0);
        rm[0][0]=1.0; rm[1][0]=0.0;     rm[2][0]=0.0;       rm[3][0]=0.0;
        rm[0][1]=0.0; rm[1][1]=cos(rx); rm[2][1]=-sin(rx);  rm[3][1]=0.0;
        rm[0][2]=0.0; rm[1][2]=sin(rx); rm[2][2]=cos(rx);   rm[3][2]=0.0;
        rm[0][3]=0.0; rm[1][3]=0.0;     rm[2][3]=0.0;       rm[3][3]=1.0;
        return rm;
    }

    mat4 rmY(float rotationY)
    {
        float ry = rotationY/180.0 * PI;
        mat4 rm = mat4(0.0);
        rm[0][0]=cos(ry);   rm[1][0]=0.0;   rm[2][0]=sin(ry);   rm[3][0]=0.0;
        rm[0][1]=0.0;       rm[1][1]=1.0;   rm[2][1]=0.0;       rm[3][1]=0.0;
        rm[0][2]=-sin(ry);  rm[1][2]=0.0;   rm[2][2]=cos(ry);   rm[3][2]=0.0;
        rm[0][3]=0.0;       rm[1][3]=0.0;   rm[2][3]=0.0;       rm[3][3]=1.0;
        return rm;
    }

    mat4 rmZ(float rotationZ)
    {
        float rz = rotationZ/180.0 * PI;
        mat4 rm = mat4(0.0);
        rm[0][0]=cos(rz);   rm[1][0]=-sin(rz);  rm[2][0]=0.0;   rm[3][0]=0.0;
        rm[0][1]=sin(rz);   rm[1][1]=cos(rz);   rm[2][1]=0.0;   rm[3][1]=0.0;
        rm[0][2]=0.0;       rm[1][2]=0.0;       rm[2][2]=1.0;   rm[3][2]=0.0;
        rm[0][3]=0.0;       rm[1][3]=0.0;       rm[2][3]=0.0;   rm[3][3]=1.0;
        return rm;
    }
    
    mat4 pmFrustum(float left,     float right,
                   float bottom,   float top,
                   float near,     float far)
    {
        mat4 pm = mat4(0.0);
        pm[0][0]=(2*near)/(right-left);
        pm[1][1]=(2*near)/(top-bottom);
        
        pm[0][2]=(right+left)/(right-left);
        pm[1][2]=(top+bottom)/(top-bottom);
        pm[2][2]=-(far+near)/(far-near);
        pm[3][2]=(-1.0);
        
        pm[2][3]=-(2*far*near)/(far-near);
        pm[3][3]=1.0;
        return pm;
    }

    void main (){
        vec4 vertex = vec4(vertexPos, 1.0);

        float fovVertical = 70.0;
        float aspectRatio = 800.0/600.0;
        float zNear = 1.0;
        float zFar = 100.0;

        float halfW = tan(0.5 * (fovVertical/180 * PI));
        float halfH = halfW / aspectRatio;
        mat4 projection = pmFrustum(-halfW, halfW, -halfH, halfH, zNear, zFar);

        mat4 rotationX = rmX(rotate.x);
        mat4 rotationY = rmY(rotate.y);
        vec3 translatev = vec3(0.0, -2.0, -10.0);
        mat4 viewzoom = tmV(translatev);
        
        gl_Position = projection * (viewzoom * (rotationX * rotationY)) * vertex;
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

fn build_fragment_shader(what:u8) -> GLuint {
    let fshader:GLuint = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

    let source0: &CStr  = CStr::from_bytes_with_nul(b"#version 330 core
            out vec4 fragColor;
            void main (){
            fragColor = vec4(0.2, 0.2, 0.2, 1.0);
            }\0").unwrap();
    let source1: &CStr  = CStr::from_bytes_with_nul(b"#version 330 core
            out vec4 fragColor;
            void main (){
            fragColor = vec4(0.6, 0.6, 0.6, 1.0);
            }\0").unwrap();
    match what {
        0=>unsafe { gl::ShaderSource(fshader, 1, &source0.as_ptr(), std::ptr::null()); }
        1=>unsafe { gl::ShaderSource(fshader, 1, &source1.as_ptr(), std::ptr::null()); }
        _=>unsafe { gl::ShaderSource(fshader, 1, &source0.as_ptr(), std::ptr::null()); }
    }

    unsafe {
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