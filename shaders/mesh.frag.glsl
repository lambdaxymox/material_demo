#version 330 core

/// Material properties for the Blinn-Phong shader model.
struct Material {
    vec3 ambient,
    vec3 diffuse,
    vec3 specular,
    float specular_exponent,
}

// A point light with specular, diffuse, and ambient components. Each component is 
// specified in units of 'intensity' which is an unspecified unit of the light's radiant
// exitance. Here the three vectors approximate the spectral dependence of light 'intensity'
// in terms of R, G, and B channels.
struct Light {
    // The position of the light in world space.
    vec3 position_world;
    // The ambient component of the point light.
    vec3 ambient;
    // The diffuse component of the point light.
    vec3 diffuse;
    // The specular component of the point light.
    vec3 specular;
}


in vec3 v_position_eye;
in vec2 v_tex_coord;
in vec3 v_normal_eye;

uniform mat4 model_mat;
uniform Material material;
uniform Light light;

out vec4 frag_color;


void main() {
    // Calculate the ambient part of the lighting model.
    vec3 frag_ambient = light.ambient * material.ambient;

    // Calculate the diffuse part of the lighting model.
    vec3 norm_eye = normalize(normal_eye);
    vec3 light_position_eye = vec3(view_mat * vec4(light.position_world, 1.0));
    vec3 light_dir_eye = normalize(light_position_eye - v_position_eye);
    float diff = max(dot(norm_eye, light_dir_eye), 0.0);
    vec3 frag_diffuse = light.diffuse * (diff * material.diffuse);

    // Calculate the specular part of the lighting model.
    vec3 view_dir_eye = normalize(-position_eye);
    vec3 half_vec_eye = normalize(view_dir_eye + light_dir_eye);
    float dot_specular = max(dot(half_vec_eye, norm_eye), 0.0);
    float specular_factor = pow(dot_specular, material.specular_exponent);
    vec3 frag_specular = light.specular * material.specular * specular_factor;
    
    vec3 frag_result = frag_ambient + frag_diffuse + frag_specular;
    frag_color = vec4(frag_result, 1.0);
}
