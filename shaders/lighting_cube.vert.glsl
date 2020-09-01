#version 330 core
struct Camera {
    mat4 proj_mat;
    mat4 view_mat;
};

layout (location = 0) in vec3 v_pos;

uniform mat4 model_mat;
uniform Camera camera;


void main() {
    gl_Position = camera.proj_mat * camera.view_mat * model_mat * vec4(v_pos, 1.0);
}
