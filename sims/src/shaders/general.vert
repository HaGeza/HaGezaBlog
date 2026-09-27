#version 330
precision highp float;

in vec3 position;
in vec4 color0;
in vec4 normal0;

uniform mat4 Model;
uniform mat4 Projection;

out vec4 color;
out vec3 normal;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    color = color0;
    normal = normalize(transpose(inverse(mat3(Model))) * normal0.xyz);
}
