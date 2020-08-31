extern crate glfw;
extern crate gdmath;
extern crate log;
extern crate file_logger;
extern crate mini_obj;


mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod backend;
mod camera;

use camera::{
    CameraSpecification,
    CameraKinematics,
    CameraAttitude,
    Camera
};

use gdmath::{
    Degrees,
    Quaternion,
    Matrix4,
    Storage,
    Vector3,
};
use glfw::{Action, Context, Key};
use gl::types::{
    GLfloat,
    GLint,
    GLuint, 
    GLvoid, 
    GLsizeiptr
};
use log::{info};
use mini_obj::ObjMesh;
use std::io;
use std::mem;
use std::ptr;


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

fn create_camera(width: u32, height: u32) -> Camera<f32> {
    let near = 0.1;
    let far = 100.0;
    let fovy = Degrees(67.0);
    let aspect = width as f32 / height as f32;
    let spec = CameraSpecification::new(near, far, fovy, aspect);

    let speed = 5.0;
    let yaw_speed = 50.0;
    let position = gdmath::vec3((0.0, 0.0, 10.0));
    let forward = gdmath::vec4((0.0, 0.0, 1.0, 0.0));
    let right = gdmath::vec4((1.0, 0.0, 0.0, 0.0));
    let up  = gdmath::vec4((0.0, 1.0, 0.0, 0.0));
    let axis = Quaternion::new(0.0, 0.0, 0.0, -1.0);
    let kinematics = CameraKinematics::new(speed, yaw_speed, position, forward, right, up, axis);

    Camera::new(spec, kinematics)
}

struct Material {
    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
    specular_exponent: f32,
}

struct Light {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub specular_exponent: f32,
    pub position_world: Vector3<f32>,
}

struct Uniforms {
    model_mat: Matrix4<f32>,
    camera: Camera<f32>,

}

fn send_to_gpu_uniforms(shader: GLuint, uniforms: Uniforms) {
    let model_mat_loc = unsafe {
        gl::GetUniformLocation(shader, backend::gl_str("model_mat").as_ptr())
    };
    debug_assert!(model_mat_loc > -1);
    let camera_proj_mat_loc = unsafe {
        gl::GetUniformLocation(shader, backend::gl_str("camera.proj_mat").as_ptr())
    };
    debug_assert!(camera_proj_mat_loc > -1);
    let camera_view_mat_loc = unsafe {
        gl::GetUniformLocation(shader, backend::gl_str("camera.view_mat").as_ptr())
    };
    debug_assert!(camera_view_mat_loc > -1);
    
    unsafe {
        gl::UseProgram(shader);
        gl::UniformMatrix4fv(model_mat_loc, 1, gl::FALSE, uniforms.model_mat.as_ptr());
        gl::UniformMatrix4fv(camera_proj_mat_loc, 1, gl::FALSE, uniforms.camera.proj_mat.as_ptr());
        gl::UniformMatrix4fv(camera_proj_mat_loc, 1, gl::FALSE, uniforms.camera.view_mat.as_ptr());
    }
}

/*
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
*/
fn send_to_gpu_mesh(shader: GLuint, mesh: &ObjMesh) -> (GLuint, GLuint, GLuint, GLuint) {
    let v_pos_loc = unsafe {
        gl::GetAttribLocation(shader, backend::gl_str("v_pos").as_ptr())
    };
    debug_assert!(v_pos_loc > -1);
    let v_pos_loc = v_pos_loc as u32;

    let v_tex_loc = unsafe {
        gl::GetAttribLocation(shader, backend::gl_str("v_tex").as_ptr())
    };
    debug_assert!(v_tex_loc > -1);
    let v_tex_loc = v_tex_loc as u32;

    let v_norm_loc = unsafe {
        gl::GetAttribLocation(shader, backend::gl_str("v_norm").as_ptr())
    };
    debug_assert!(v_norm_loc > -1);
    let v_norm_loc = v_norm_loc as u32;

    let mut v_pos_vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut v_pos_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, v_pos_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (3 * mem::size_of::<GLfloat>() * mesh.points.len()) as GLsizeiptr,
            mesh.points.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );
    }
    debug_assert!(v_pos_vbo > 0);

    let mut v_tex_vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut v_tex_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, v_tex_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            // (2 * mem::size_of::<GLfloat>() * mesh.tex_coords.len()) as GLsizeiptr,
            mesh.tex_coords.len_bytes() as GLsizeiptr,
            mesh.tex_coords.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );
    }
    debug_assert!(v_tex_vbo > 0);

    let mut v_norm_vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut v_norm_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, v_norm_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            // (3 * mem::size_of::<GLfloat>() * mesh.points.len()) as GLsizeiptr,
            mesh.normals.len_bytes() as GLsizeiptr,
            mesh.normals.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );
    }
    debug_assert!(v_norm_vbo > 0);

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, v_pos_vbo);
        gl::VertexAttribPointer(v_pos_loc, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, v_tex_vbo);
        gl::VertexAttribPointer(v_tex_loc, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, v_norm_vbo);
        gl::VertexAttribPointer(v_norm_loc, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(v_pos_loc);
        gl::EnableVertexAttribArray(v_tex_loc);
        gl::EnableVertexAttribArray(v_norm_loc);
    }
    debug_assert!(vao > 0);

    (vao, v_pos_vbo, v_tex_vbo, v_norm_vbo)
}

#[derive(Copy, Clone)]
struct ShaderSource {
    vert_name: &'static str,
    vert_source: &'static str,
    frag_name: &'static str,
    frag_source: &'static str,
}

fn create_shader_source() -> ShaderSource {
    let vert_source = include_str!("../shaders/mesh.vert.glsl");
    let frag_source = include_str!("../shaders/mesh.frag.glsl");
    
    ShaderSource {
        vert_name: "mesh.vert.glsl",
        vert_source: vert_source,
        frag_name: "mesh.frag.glsl",
        frag_source: frag_source,
    }
}
            
fn send_to_gpu_shaders(context: &mut backend::OpenGLContext, source: ShaderSource) -> GLuint {
    let mut vert_reader = io::Cursor::new(source.vert_source);
    let mut frag_reader = io::Cursor::new(source.frag_source);
    let result = backend::compile_from_reader(
        &mut vert_reader, source.vert_name,
        &mut frag_reader, source.frag_name
    );
    let shader = match result {
        Ok(value) => value,
        Err(e) => {
            panic!("Could not compile shaders. Got error: {}", e);
        }
    };
    debug_assert!(shader > 0);

    shader
}

/// Initialize the logger.
fn init_logger(log_file: &str) {
    file_logger::init(log_file).expect("Failed to initialize logger.");
}

/// Create and OpenGL context.
fn init_gl(width: u32, height: u32) -> backend::OpenGLContext {
    let context = match backend::start_gl(width, height) {
        Ok(val) => val,
        Err(e) => {
            panic!("Failed to Initialize OpenGL context. Got error: {}", e);
        }
    };

    context
}

fn main() {
    let mesh = load_mesh();
    init_logger("arcball_demo.log");
    let mut camera = create_camera(WIDTH, HEIGHT);
    let mut context = init_gl(WIDTH, HEIGHT);
    let shader_source = create_shader_source();
    let shader = send_to_gpu_shaders(&mut context, shader_source);
    let (
        vao, 
        v_pos_vbo, 
        v_tex_vbo, 
        v_norm_vbo) = send_to_gpu_mesh(shader, &mesh);

    while !context.window.should_close() {
        let elapsed_seconds = context.update_timers();
        context.update_fps_counter();
        context.glfw.poll_events();
        match context.window.get_key(Key::Escape) {
            Action::Press | Action::Repeat => {
                context.window.set_should_close(true);
            }
            _ => {}
        }

        // Camera control keys.
        let mut cam_moved = false;
        let mut move_to = gdmath::vec3((0.0, 0.0, 0.0));
        let mut cam_attitude = CameraAttitude::new(0.0, 0.0, 0.0);
        match context.window.get_key(Key::A) {
            Action::Press | Action::Repeat => {
                move_to.x -= camera.speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
            }
        match context.window.get_key(Key::D) {
            Action::Press | Action::Repeat => {
                move_to.x += camera.speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::Q) {
            Action::Press | Action::Repeat => {
                move_to.y += camera.speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::E) {
            Action::Press | Action::Repeat => {
                move_to.y -= camera.speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::W) {
            Action::Press | Action::Repeat => {
                move_to.z -= camera.speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::S) {
            Action::Press | Action::Repeat => {
                move_to.z += camera.speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::Left) {
            Action::Press | Action::Repeat => {
                cam_attitude.yaw += camera.yaw_speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::Right) {
            Action::Press | Action::Repeat => {
                cam_attitude.yaw -= camera.yaw_speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::Up) {
            Action::Press | Action::Repeat => {
                cam_attitude.pitch += camera.yaw_speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::Down) {
            Action::Press | Action::Repeat => {
                cam_attitude.pitch -= camera.yaw_speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::Z) {
            Action::Press | Action::Repeat => {
                cam_attitude.roll -= camera.yaw_speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::C) {
            Action::Press | Action::Repeat => {
                cam_attitude.roll += camera.yaw_speed * (elapsed_seconds as GLfloat);
                cam_moved = true;
            }
            _ => {}
        }
        match context.window.get_key(Key::Escape) {
            Action::Press | Action::Repeat => {
                context.window.set_should_close(true);
            }
            _ => {}
        }

        if cam_moved {
            camera.update(move_to, cam_attitude);
        }

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &CLEAR_COLOR[0] as *const GLfloat);
            gl::ClearBufferfv(gl::DEPTH, 0, &CLEAR_DEPTH[0] as *const GLfloat);
            gl::Viewport(0, 0, WIDTH as GLint, HEIGHT as GLint);
        }

        context.window.swap_buffers();
    }

    info!("END LOG");
}
