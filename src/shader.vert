#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec3 a_normal;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec3 v_normal;
layout(location=2) out vec3 v_position;

layout(location=5) in vec4 model_matrix_0;
layout(location=6) in vec4 model_matrix_1;
layout(location=7) in vec4 model_matrix_2;
layout(location=8) in vec4 model_matrix_3;

layout(set=1, binding=0)
uniform Uniforms {
    vec4 u_view_pos;
    mat4 u_view;
    mat4 u_proj;
};

void main() {
    mat4 model_matrix = mat4(
        model_matrix_0,
        model_matrix_1,
        model_matrix_2,
        model_matrix_3
    );
    mat3 normal_matrix = mat3(transpose(inverse(model_matrix)));

    v_normal = normal_matrix * a_normal.xyz;
    v_tex_coords = a_tex_coords;
    vec4 model_space = model_matrix * vec4(a_position.xyz, 1.0);
    v_position = model_space.xyz;
    gl_Position = u_proj * u_view * model_space;
}
