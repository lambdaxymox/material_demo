extern crate glfw;
extern crate gdmath;
extern crate log;
extern crate file_logger;
extern crate mini_obj;


mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod backend;

use glfw::{Action, Context, Key};
use gl::types::{GLfloat, GLint, GLuint, GLvoid, GLsizeiptr};
use log::{info};
use mini_obj::ObjMesh;
use tex_atlas::TextureAtlas2D;
use std::io;


// OpenGL extension constants.
const GL_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FF;

// Default value for the color buffer.
const CLEAR_COLOR: [f32; 4] = [0.2_f32, 0.2_f32, 0.2_f32, 1.0_f32];
// Default value for the depth buffer.
const CLEAR_DEPTH: [f32; 4] = [1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32];

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn load_mesh() -> ObjMesh {
    let buffer = include_bytes!("../assets/teapot.obj");
    let mesh = mini_obj::load_from_memory(buffer).unwrap();

    mesh
}


/// Load texture image into the GPU.
fn send_to_gpu_texture(atlas: &TextureAtlas2D, wrapping_mode: GLuint) -> Result<GLuint, String> {
    let mut tex = 0;
    unsafe {
        gl::GenTextures(1, &mut tex);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGBA as i32, atlas.width as i32, atlas.height as i32, 0,
            gl::RGBA, gl::UNSIGNED_BYTE,
            atlas.as_ptr() as *const GLvoid
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping_mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping_mode as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
    }
    debug_assert!(tex > 0);

    let mut max_aniso = 0.0;
    unsafe {
        gl::GetFloatv(GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT, &mut max_aniso);
        // Set the maximum!
        gl::TexParameterf(gl::TEXTURE_2D, GL_TEXTURE_MAX_ANISOTROPY_EXT, max_aniso);
    }

    Ok(tex)
}

#[derive(Copy, Clone)]
struct ShaderSource {
    vert_name: &'static str,
    vert_source: &'static str,
    frag_name: &'static str,
    frag_source: &'static str,
}

fn send_to_gpu_shaders(game: &mut backend::GLState, source: ShaderSource) -> GLuint {
    let mut vert_reader = io::Cursor::new(source.vert_source);
    let mut frag_reader = io::Cursor::new(source.frag_source);
    let sp = backend::create_program_from_reader(
        game,
        &mut vert_reader, source.vert_name,
        &mut frag_reader, source.frag_name
    ).unwrap();
    debug_assert!(sp > 0);

    sp
}

/// Initialize the logger.
fn init_logger(log_file: &str) {
    file_logger::init(log_file).expect("Failed to initialize logger.");
}

/// Create and OpenGL context.
fn init_gl(width: u32, height: u32) -> backend::GLState {
    let gl_state = match backend::start_gl(width, height) {
        Ok(val) => val,
        Err(e) => {
            panic!("Failed to Initialize OpenGL context. Got error: {}", e);
        }
    };

    gl_state
}

fn main() {
    let mesh = load_mesh();
    init_logger("arcball_demo.log");
    let mut gl_state = init_gl(WIDTH, HEIGHT);
    while !gl_state.window.should_close() {
        let elapsed = backend::update_timers(&mut gl_state);
        backend::update_fps_counter(&mut gl_state);
        gl_state.glfw.poll_events();
        match gl_state.window.get_key(Key::Escape) {
            Action::Press | Action::Repeat => {
                gl_state.window.set_should_close(true);
            }
            _ => {}
        }

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &CLEAR_COLOR[0] as *const GLfloat);
            gl::ClearBufferfv(gl::DEPTH, 0, &CLEAR_DEPTH[0] as *const GLfloat);
            gl::Viewport(0, 0, WIDTH as GLint, HEIGHT as GLint);
        }

        gl_state.window.swap_buffers();
    }

    info!("END LOG");
}
