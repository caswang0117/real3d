#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec3 v_normal;
layout(location=2) in vec3 v_position;

layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;
layout(set=1, binding=0)
uniform Uniforms {
    vec3 u_view_position; // unused
    mat4 u_view;
    mat4 u_proj;
};

struct Light {
  vec4 pos;
  vec4 color;
  // vec4 dir;
};

layout(set=2, binding=0)
uniform Lights {
    Light lights[10];
};
layout(set=2, binding=1)
uniform LightsAmbient {
    float ambient;
};


void main() {
  vec3 normal = normalize(v_normal);
  vec4 object_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
  vec3 view_dir = normalize(u_view_position - v_position);

  vec3 result = ambient*object_color.xyz;
  for (int i = 0; i < 10; i++) {
    float light_ambient = 0.1;
    // Point-light specific; change if directional lights, spotlights are used
    // to branch on e.g. position.w == 0 (directional) or direction.w == 0 (point) or else spot
    vec3 light_color = lights[i].color.xyz;
    vec3 light_position = lights[i].pos.xyz;
    vec3 light_dir = normalize(light_position - v_position);
    float diffuse_strength = max(dot(normal, light_dir), 0.0);
    vec3 diffuse_color = light_color * diffuse_strength;
    vec3 ambient_color = light_color * light_ambient;
    vec3 half_dir = normalize(view_dir + light_dir);
    float specular_strength = pow(max(dot(normal, half_dir), 0.0), 32);
    vec3 specular_color = specular_strength * light_color;
    result += (ambient_color + diffuse_color + specular_color) * object_color.xyz;
  }
  f_color = vec4(result, object_color.a);
}
