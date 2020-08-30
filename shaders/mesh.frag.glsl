#version 330 core

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
uniform sampler2D tex;
uniform Light light;

out vec4 frag_color;


void main() {
    // TODO: we are using the same reflectance for all material components of the model
    // because we have not yet defined separate texture maps for each component in the material
    // properties.
    vec3 reflectance = vec3 (texture(tex, tex_coord));
    vec3 ref_ambient = reflectance;
    vec3 ref_diffuse = reflectance;
    vec3 ref_specular = reflectance;

    // Calculate the ambient part of the lighting model.
    vec3 norm_eye = normalize(normal_eye);
    vec3 frag_ambient = light.ambient * ref_ambient;

    // Calculate the diffuse part of the lighting model.
    vec3 light_position_eye = vec3(view_mat * vec4(light.position_world, 1.0));
    vec3 light_dir_eye = normalize(light_position_eye - v_position_eye);
    float diff = max(dot(norm_eye, light_dir_eye), 0.0);
    vec3 frag_diffuse = light.diffuse * (diff * ref_diffuse);

    // Calculate the specular part of the lighting model.
    vec3 view_dir_eye = normalize(-position_eye);
    vec3 half_vec_eye = normalize(view_dir_eye + light_dir_eye);
    float dot_specular = max(dot(half_vec_eye, norm_eye), 0.0);
    float specular_factor = pow(dot_specular, light.specular_exponent);
    vec3 frag_specular = light.specular * ref_specular * specular_factor;
    
    vec3 frag_result = frag_ambient + frag_diffuse + frag_specular;
    frag_color = vec4(result, 1.0);
}
