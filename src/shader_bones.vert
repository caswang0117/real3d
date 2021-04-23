#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec3 a_normal;
layout(location=3) in uint bone_ids;
layout(location=4) in vec4 bone_weights;

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

struct Bone {
    vec4 pos;
    vec4 rot;
};

layout(set=3, binding=0)
uniform Bones {
    Bone bones[128];
};

// This shader uses uniforms for bone positions so it won't work with
// instancing.  In a real application you'd prefer to pass the bone
// positions as instance data but you can't really, so you need to
// encode bone positions of each instance into a texture or something
// and sample it.  Or even better, put the animation data into a
// texture and use animation state as instance data, then figure out
// vertex transform from that.


vec4 quat_mul_s(vec4 q1, float s)
{
  vec4 q2 = q1*s;
  float m = sqrt(q2.x*q2.x+q2.y*q2.y+q2.z*q2.z+q2.w*q2.w);
  return q2/m;
}

vec3 quat_rot(vec4 q, vec3 v) {
  return v + 2.0 * cross(q.xyz, cross(q.xyz, v) + q.w * v);
}

void main() {
    mat4 model_matrix = mat4(
        model_matrix_0,
        model_matrix_1,
        model_matrix_2,
        model_matrix_3
    );
    mat3 normal_matrix = mat3(transpose(inverse(model_matrix)));

    vec3 new_vertex = a_position;
    vec3 new_normal = a_normal;
    for (int idx=0; idx < 4; idx++) {
      int index = int(bone_ids >> (8*(3-idx)) & 0x000000FF);
      float weight = bone_weights[idx];
      // the bone is a position offset and a rotation.
      // so adjust the position...
      new_vertex += bones[index].pos.xyz * weight;
      // and rotate!
      vec4 rot = quat_mul_s(bones[index].rot, weight);
      new_vertex = quat_rot(rot, new_vertex);
      new_normal = quat_rot(rot, new_normal);
    }
    v_normal = normal_matrix * new_normal;
    v_tex_coords = a_tex_coords;
    vec4 model_space = model_matrix * vec4(new_vertex.xyz, 1.0);
    v_position = model_space.xyz;
    gl_Position = u_proj * u_view * model_space;
}
