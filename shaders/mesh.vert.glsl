#version 330 core

struct Camera {
    // The transformation converting from camera space to 
    // the canonical view volume.
    mat4 proj_mat;
    // The coordinate transformation for converting from
    // world space to camera space.
    mat4 view_mat;
};

struct FragData {
    // The vertex position for a vertex in camera space.
    vec3 position_eye;
    // The normal vector for a fragment in camera space.
    vec3 normal_eye;
};

layout(location = 0) in vec3 v_pos;
layout(location = 1) in vec3 v_norm;

// The coordinate transformation placing an object from model 
// space to world space.
uniform mat4 model_mat;
uniform Camera camera;

out FragData vertex_data;


void main() {
    vertex_data.position_eye = vec3(camera.view_mat * model_mat * vec4(v_pos, 1.0));
    vertex_data.normal_eye = vec3(camera.view_mat * model_mat * vec4(v_norm, 0.0));

    gl_Position = camera.proj_mat * vec4(vertex_data.position_eye, 1.0);
}
