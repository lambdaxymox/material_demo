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
mod light;
mod material;

use backend::{
    OpenGLContext,
};
use camera::{
    SimpleCameraMovement,
    CameraMovement,
    CameraSpecification,
    CameraKinematics,
    Camera
};
use light::PointLight;
use material::Material;
use gdmath::{
    Degrees,
    Quaternion,
    Magnitude,
    Matrix4,
    Storage,
    Vector3,
    One,
    Zero,
};
use glfw::{
    Action, 
    Context, 
    Key
};
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


// Default value for the color buffer.
const CLEAR_COLOR: [f32; 4] = [0.2_f32, 0.2_f32, 0.2_f32, 1.0_f32];
// Default value for the depth buffer.
const CLEAR_DEPTH: [f32; 4] = [1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32];

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn create_mesh() -> ObjMesh {
    let buffer = include_bytes!("../assets/teapot.obj");
    let mesh = mini_obj::load_from_memory(buffer).unwrap();

    mesh
}

fn create_box_mesh() -> ObjMesh {
    let points: Vec<[f32; 3]> = vec![
        [-0.5, -0.5, -0.5], [ 0.5, -0.5, -0.5], [ 0.5,  0.5, -0.5],
        [ 0.5,  0.5, -0.5], [-0.5,  0.5, -0.5], [-0.5, -0.5, -0.5],
        [-0.5, -0.5,  0.5], [ 0.5, -0.5,  0.5], [ 0.5,  0.5,  0.5],  
        [ 0.5,  0.5,  0.5], [-0.5,  0.5,  0.5], [-0.5, -0.5,  0.5],
        [-0.5,  0.5,  0.5], [-0.5,  0.5, -0.5], [-0.5, -0.5, -0.5], 
        [-0.5, -0.5, -0.5], [-0.5, -0.5,  0.5], [-0.5,  0.5,  0.5], 
        [ 0.5,  0.5,  0.5], [ 0.5,  0.5, -0.5], [ 0.5, -0.5, -0.5], 
        [ 0.5, -0.5, -0.5], [ 0.5, -0.5,  0.5], [ 0.5,  0.5,  0.5],
        [-0.5, -0.5, -0.5], [ 0.5, -0.5, -0.5], [ 0.5, -0.5,  0.5],  
        [ 0.5, -0.5,  0.5], [-0.5, -0.5,  0.5], [-0.5, -0.5, -0.5],
        [-0.5,  0.5, -0.5], [ 0.5,  0.5, -0.5], [ 0.5,  0.5,  0.5], 
        [ 0.5,  0.5,  0.5], [-0.5,  0.5,  0.5], [-0.5,  0.5, -0.5],  
    ];
    let tex_coords = vec![];
    let normals = vec![
        [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0],
        [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0], [ 0.0,  0.0, -1.0],
        [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0],
        [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0], [ 0.0,  0.0,  1.0],
        [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0],
        [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0], [-1.0,  0.0,  0.0],
        [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0],
        [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0], [ 1.0,  0.0,  0.0],
        [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0],
        [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0], [ 0.0, -1.0,  0.0],
        [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0],
        [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0], [ 0.0,  1.0,  0.0],
    ];

    ObjMesh::new(points, tex_coords, normals)
}

fn create_camera(width: u32, height: u32) -> Camera<f32> {
    let near = 0.1;
    let far = 100.0;
    let fovy = Degrees(67.0);
    let aspect = width as f32 / height as f32;
    let spec = CameraSpecification::new(near, far, fovy, aspect);

    let speed = 5.0;
    let yaw_speed = 50.0;
    let position = gdmath::vec3((0.0, 0.0, 3.0));
    let forward = gdmath::vec4((0.0, 0.0, 1.0, 0.0));
    let right = gdmath::vec4((1.0, 0.0, 0.0, 0.0));
    let up  = gdmath::vec4((0.0, 1.0, 0.0, 0.0));
    let axis = Quaternion::new(0.0, 0.0, 0.0, -1.0);
    let kinematics = CameraKinematics::new(speed, yaw_speed, position, forward, right, up, axis);

    Camera::new(spec, kinematics)
}

fn create_light() -> PointLight<f32> {
    let ambient = Vector3::new(0.2, 0.2, 0.2);
    let diffuse = Vector3::new(0.5, 0.5, 0.5);
    let specular = Vector3::new(1.0, 1.0, 1.0);
    
    PointLight::new(ambient, diffuse, specular)
}

struct LightKinematics {
    scene_center: Vector3<f32>,
    radial_speed: f32,
    center_of_oscillation: Vector3<f32>,
    radius_of_oscillation: f32,
    position: Vector3<f32>,
    radial_unit_velocity: f32,
    orbital_axis: Vector3<f32>,
}

impl LightKinematics {
    pub fn new(
        scene_center: Vector3<f32>, 
        radial_speed: f32, 
        center_of_oscillation: Vector3<f32>, 
        radius_of_oscillation: f32,
        orbital_axis: Vector3<f32>) -> LightKinematics {
        
        let radial_unit_velocity = 1.0;
        let position = center_of_oscillation;
        LightKinematics {
            scene_center: scene_center,
            radial_speed: radial_speed,
            center_of_oscillation: center_of_oscillation,
            radius_of_oscillation: radius_of_oscillation,
            position: position,
            radial_unit_velocity: radial_unit_velocity,
            orbital_axis: orbital_axis.normalize(),
        }
    }

    #[inline]
    fn position(&self) -> Vector3<f32> {
        self.position
    }

    fn update(&mut self, elapsed_seconds: f32) {
        self.radial_unit_velocity = if self.radial_unit_velocity < 0.0 { -1.0 } else { 1.0 };
        let radius_center_of_oscillation = (self.center_of_oscillation - self.scene_center).magnitude();
        let radial_vector: Vector3<f32> = (self.position - self.scene_center).normalize();
        let radius_perihelion = radius_center_of_oscillation - self.radius_of_oscillation;
        let radius_aphelion = radius_center_of_oscillation + self.radius_of_oscillation;
        let mut distance_from_scene_center = (self.position - self.scene_center).magnitude();
        distance_from_scene_center = distance_from_scene_center + (self.radial_speed * elapsed_seconds) * self.radial_unit_velocity;
        if distance_from_scene_center < radius_perihelion {
            distance_from_scene_center = radius_perihelion;
            self.radial_unit_velocity = 1.0;
        } else if distance_from_scene_center > radius_aphelion {
            distance_from_scene_center = radius_aphelion;
            self.radial_unit_velocity = -1.0;
        }
    
        self.position = distance_from_scene_center * radial_vector;
    }

    fn model_mat(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
    }
}

fn send_to_gpu_uniforms_mesh(shader: GLuint, model_mat: &Matrix4<f32>) {
    let model_mat_loc = unsafe {
        gl::GetUniformLocation(shader, backend::gl_str("model_mat").as_ptr())
    };
    debug_assert!(model_mat_loc > -1);
    
    unsafe {
        gl::UseProgram(shader);
        gl::UniformMatrix4fv(model_mat_loc, 1, gl::FALSE, model_mat.as_ptr());
    }
}

fn send_to_gpu_uniforms_camera(shader: GLuint, camera: &Camera<f32>) {
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
        gl::UniformMatrix4fv(camera_proj_mat_loc, 1, gl::FALSE, camera.proj_mat.as_ptr());
        gl::UniformMatrix4fv(camera_view_mat_loc, 1, gl::FALSE, camera.view_mat.as_ptr());
    }
}

fn send_to_gpu_uniforms_light(shader: GLuint, light: &PointLight<f32>, position_world: Vector3<f32>) {
    let light_position_world_loc = unsafe {
        gl::GetUniformLocation(shader, backend::gl_str("light.position_world").as_ptr())
    };
    debug_assert!(light_position_world_loc > -1);
    let light_ambient_loc = unsafe {
        gl::GetUniformLocation(shader, backend::gl_str("light.ambient").as_ptr())
    };
    debug_assert!(light_ambient_loc > -1);
    let light_diffuse_loc = unsafe {
        gl::GetUniformLocation(shader, backend::gl_str("light.diffuse").as_ptr())
    };
    debug_assert!(light_diffuse_loc > -1);
    let light_specular_loc = unsafe { 
        gl::GetUniformLocation(shader, backend::gl_str("light.specular").as_ptr())
    };
    debug_assert!(light_specular_loc > -1);

    unsafe {
        gl::UseProgram(shader);
        gl::Uniform3fv(light_position_world_loc, 1, position_world.as_ptr());
        gl::Uniform3fv(light_ambient_loc, 1, light.ambient.as_ptr());
        gl::Uniform3fv(light_diffuse_loc, 1, light.diffuse.as_ptr());
        gl::Uniform3fv(light_specular_loc, 1, light.specular.as_ptr());
    }
}

fn send_to_gpu_uniforms_material(shader: GLuint, material: &Material<f32>) {
    let material_ambient_loc = unsafe {
        gl::GetUniformLocation(shader, backend::gl_str("material.ambient").as_ptr())
    };
    debug_assert!(material_ambient_loc > -1);
    let material_diffuse_loc = unsafe {
        gl::GetUniformLocation(shader, backend::gl_str("material.diffuse").as_ptr())
    };
    debug_assert!(material_diffuse_loc > -1);
    let material_specular_loc = unsafe {
        gl::GetUniformLocation(shader, backend::gl_str("material.specular").as_ptr())
    };
    debug_assert!(material_specular_loc > -1);
    let material_specular_exponent_loc = unsafe { 
        gl::GetUniformLocation(shader, backend::gl_str("material.specular_exponent").as_ptr())
    };
    debug_assert!(material_specular_exponent_loc > -1);

    unsafe {
        gl::UseProgram(shader);
        gl::Uniform3fv(material_ambient_loc, 1, material.ambient.as_ptr());
        gl::Uniform3fv(material_diffuse_loc, 1, material.diffuse.as_ptr());
        gl::Uniform3fv(material_specular_loc, 1, material.specular.as_ptr());
        gl::Uniform1f(material_specular_exponent_loc, material.specular_exponent);
    }
}

fn send_to_gpu_mesh(shader: GLuint, mesh: &ObjMesh) -> (GLuint, GLuint, GLuint) {
    let v_pos_loc = unsafe {
        gl::GetAttribLocation(shader, backend::gl_str("v_pos").as_ptr())
    };
    debug_assert!(v_pos_loc > -1);
    let v_pos_loc = v_pos_loc as u32;

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

    let mut v_norm_vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut v_norm_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, v_norm_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
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
        gl::BindBuffer(gl::ARRAY_BUFFER, v_norm_vbo);
        gl::VertexAttribPointer(v_norm_loc, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(v_pos_loc);
        gl::EnableVertexAttribArray(v_norm_loc);
    }
    debug_assert!(vao > 0);

    (vao, v_pos_vbo, v_norm_vbo)
}

fn send_to_gpu_light_mesh(shader: GLuint, mesh: &ObjMesh) -> (GLuint, GLuint) {
    let v_pos_loc = unsafe {
        gl::GetAttribLocation(shader, backend::gl_str("v_pos").as_ptr())
    };
    debug_assert!(v_pos_loc > -1);
    let v_pos_loc = v_pos_loc as u32;

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

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, v_pos_vbo);
        gl::VertexAttribPointer(v_pos_loc, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(v_pos_loc);
    }
    debug_assert!(vao > 0);

    (vao, v_pos_vbo)
}

#[derive(Copy, Clone)]
struct ShaderSource {
    vert_name: &'static str,
    vert_source: &'static str,
    frag_name: &'static str,
    frag_source: &'static str,
}

fn create_mesh_shader_source() -> ShaderSource {
    let vert_source = include_str!("../shaders/mesh.vert.glsl");
    let frag_source = include_str!("../shaders/mesh.frag.glsl");
    
    ShaderSource {
        vert_name: "mesh.vert.glsl",
        vert_source: vert_source,
        frag_name: "mesh.frag.glsl",
        frag_source: frag_source,
    }
}

fn create_light_shader_source() -> ShaderSource {
    let vert_source = include_str!("../shaders/lighting_cube.vert.glsl");
    let frag_source = include_str!("../shaders/lighting_cube.frag.glsl");

    ShaderSource {
        vert_name: "lighting_cube.vert.glsl",
        vert_source: vert_source,
        frag_name: "lighting_cube.frag.glsl",
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
    let context = match backend::start_opengl(width, height) {
        Ok(val) => val,
        Err(e) => {
            panic!("Failed to Initialize OpenGL context. Got error: {}", e);
        }
    };

    context
}

/// The GLFW frame buffer size callback function. This is normally set using 
/// the GLFW `glfwSetFramebufferSizeCallback` function, but instead we explicitly
/// handle window resizing in our state updates on the application side. Run this function 
/// whenever the size of the viewport changes.
#[inline]
fn framebuffer_size_callback(context: &mut OpenGLContext, width: u32, height: u32) {
    context.width = width;
    context.height = height;
    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);
    }
}

fn process_input(context: &mut OpenGLContext) -> CameraMovement {
    match context.window.get_key(Key::Escape) {
        Action::Press | Action::Repeat => {
            context.window.set_should_close(true);
        }
        _ => {}
    }

    let mut movement = CameraMovement::new();
    match context.window.get_key(Key::A) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::MoveLeft;
        }
        _ => {}
        }
    match context.window.get_key(Key::D) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::MoveRight;
        }
        _ => {}
    }
    match context.window.get_key(Key::Q) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::MoveUp;
        }
        _ => {}
    }
    match context.window.get_key(Key::E) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::MoveDown;
        }
        _ => {}
    }
    match context.window.get_key(Key::W) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::MoveForward;
        }
        _ => {}
    }
    match context.window.get_key(Key::S) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::MoveBackward;
        }
        _ => {}
    }
    match context.window.get_key(Key::Left) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::YawLeft;
        }
        _ => {}
    }
    match context.window.get_key(Key::Right) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::YawRight;
        }
        _ => {}
    }
    match context.window.get_key(Key::Up) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::PitchUp;
        }
        _ => {}
    }
    match context.window.get_key(Key::Down) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::PitchDown;
        }
        _ => {}
    }
    match context.window.get_key(Key::Z) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::RollCounterClockwise;
        }
        _ => {}
    }
    match context.window.get_key(Key::C) {
        Action::Press | Action::Repeat => {
            movement += SimpleCameraMovement::RollClockwise;
        }
        _ => {}
    }

    movement
}

fn main() {
    //let mesh = create_mesh();
    //let mesh_model_mat = Matrix4::from_scale(1.0 / 50.0);
    let mesh = create_box_mesh();
    let light_mesh = create_box_mesh();
    init_logger("opengl_demo.log");
    info!("BEGIN LOG");
    let scene_center_world = Vector3::<f32>::zero();
    let mut camera = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
    let light = create_light();
    let light_orbital_axis = Vector3::new(1.0, 1.0, 1.0).normalize();
    let light_radial_speed = 3.0;
    let light_center_of_oscillation = Vector3::new(1.2, 1.0, 2.0);
    let light_radius_of_oscillation = 1.25;
    let mut light_kinematics = LightKinematics::new(
        scene_center_world, light_radial_speed, 
        light_center_of_oscillation, light_radius_of_oscillation, light_orbital_axis
    );
    let material = material::material_table()["jade"];
    let mut context = init_gl(SCREEN_WIDTH, SCREEN_HEIGHT);

    //  Load the model.
    let mesh_model_mat = Matrix4::one();
    let mesh_shader_source = create_mesh_shader_source();
    let mesh_shader = send_to_gpu_shaders(&mut context, mesh_shader_source);
    let (
        mesh_vao, 
        mesh_v_pos_vbo, 
        mesh_v_norm_vbo) = send_to_gpu_mesh(mesh_shader, &mesh);
    send_to_gpu_uniforms_mesh(mesh_shader, &mesh_model_mat);
    send_to_gpu_uniforms_camera(mesh_shader, &camera);
    send_to_gpu_uniforms_light(mesh_shader, &light, light_kinematics.position());
    send_to_gpu_uniforms_material(mesh_shader, &material);

    // Load the lighting cube model.
    let light_model_mat = light_kinematics.model_mat() * Matrix4::from_scale(0.2);
    let light_shader_source = create_light_shader_source();
    let light_shader = send_to_gpu_shaders(&mut context, light_shader_source);
    let (
        light_vao,
        light_v_pos_vbo) = send_to_gpu_light_mesh(light_shader, &light_mesh);
    send_to_gpu_uniforms_mesh(light_shader, &light_model_mat);
    send_to_gpu_uniforms_camera(light_shader, &camera);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::ClearBufferfv(gl::COLOR, 0, &CLEAR_COLOR[0] as *const GLfloat);
        gl::ClearBufferfv(gl::DEPTH, 0, &CLEAR_DEPTH[0] as *const GLfloat);
        gl::Viewport(0, 0, context.width as GLint, context.height as GLint);
    }

    while !context.window.should_close() {
        let elapsed_seconds = context.update_timers();
        context.update_fps_counter();
        context.glfw.poll_events();
        let (width, height) = context.window.get_framebuffer_size();
        if (width != context.width as i32) && (height != context.height as i32) {
            camera.update_viewport(width as u32, height as u32);
            framebuffer_size_callback(&mut context, width as u32, height as u32);
        }

        light_kinematics.update(elapsed_seconds as f32);
        let delta_movement = process_input(&mut context);
        camera.update_movement(delta_movement, elapsed_seconds as f32);
        send_to_gpu_uniforms_camera(mesh_shader, &camera);
        send_to_gpu_uniforms_camera(light_shader, &camera);
        send_to_gpu_uniforms_light(mesh_shader, &light, light_kinematics.position());

        let light_model_mat = light_kinematics.model_mat() * Matrix4::from_scale(0.2);
        send_to_gpu_uniforms_mesh(light_shader, &light_model_mat);

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &CLEAR_COLOR[0] as *const GLfloat);
            gl::ClearBufferfv(gl::DEPTH, 0, &CLEAR_DEPTH[0] as *const GLfloat);
            gl::Viewport(0, 0, context.width as GLint, context.height as GLint);
            gl::UseProgram(mesh_shader);
            gl::BindVertexArray(mesh_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, mesh.len() as i32);
            gl::UseProgram(light_shader);
            gl::BindVertexArray(light_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, light_mesh.len() as i32);
        }

        context.window.swap_buffers();
    }

    info!("END LOG");
}
