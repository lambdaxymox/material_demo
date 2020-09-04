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

// Material properties for the Blinn-Phong shader model.
struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float specular_exponent;
};

// A point light with specular, diffuse, and ambient components. Each component is 
// specified in units of 'intensity' which is an unspecified unit of the light's radiant
// exitance on the interval [0, 1]. The three vectors approximate the spectral dependence
// of light 'intensity' in terms of R, G, and B channels.
struct Light {
    // The position of the light in world space.
    vec3 position_world;
    // The ambient component of the point light.
    vec3 ambient;
    // The diffuse component of the point light.
    vec3 diffuse;
    // The specular component of the point light.
    vec3 specular;
};

const int num_lights = 3;

in FragData vertex_data;

uniform mat4 model_mat;
uniform Camera camera;
uniform Material material;
uniform Light lights[num_lights];

out vec4 frag_color;


void main() {
    vec3 frag_result = vec3(0.0, 0.0, 0.0);
    for (int i = 0; i < num_lights; i++) {
        // Calculate the ambient part of the lighting model.
        vec3 frag_ambient = lights[i].ambient * material.ambient;

        // Calculate the diffuse part of the lighting model.
        vec3 norm_eye = normalize(vertex_data.normal_eye);
        vec3 light_position_eye = vec3(camera.view_mat * vec4(lights[i].position_world, 1.0));
        vec3 light_dir_eye = normalize(light_position_eye - vertex_data.position_eye);
        float diff = max(dot(norm_eye, light_dir_eye), 0.0);
        vec3 frag_diffuse = lights[i].diffuse * (diff * material.diffuse);

        // Calculate the specular part of the lighting model.
        vec3 view_dir_eye = normalize(-vertex_data.position_eye);
        vec3 half_vec_eye = normalize(view_dir_eye + light_dir_eye);
        float dot_specular = max(dot(half_vec_eye, norm_eye), 0.0);
        float specular_factor = pow(dot_specular, material.specular_exponent);
        vec3 frag_specular = lights[i].specular * material.specular * specular_factor;

        frag_result += frag_ambient + frag_diffuse + frag_specular;
    }

    frag_color = vec4(frag_result, 1.0);
}
