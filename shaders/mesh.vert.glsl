#version 330 core

struct Camera {
    // The transformation converting from camera space to 
    // the canonical view volume.
    mat4 proj_mat;
    // The coordinate transformation for converting from
    // world space to camera space.
    mat4 view_mat;
}

in layout (location = 0) vec3 v_pos;
in layout (location = 1) vec2 v_tex;
in layout (location = 2) vec3 v_norm;

// The coordinate transformation placing an object from model 
// space to world space.
uniform mat4 model_mat;
uniform Camera camera;

// The vertex position for a vertex in camera space.
out vec3 v_position_eye;
// The texture coordinates for a vertex.
out vec3 v_tex_coord;
// The normal vector for a fragment in camera space.
out vec3 v_normal_eye;

void main() {
    v_position_eye = vec3(camera.view_mat * model_mat * vec4(v_pos, 1.0));
    v_tex_coord = v_tex;
    v_normal_eye = vec3(camera.view_mat * model_mat * vec4(v_norm, 0.0));

    gl_Position = camera.proj_mat * vec4(position_eye, 1.0);
}
