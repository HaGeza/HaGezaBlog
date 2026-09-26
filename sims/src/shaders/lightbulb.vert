#version 300 es
precision highp float;

in vec3 position;
in vec4 color0;

uniform mat4 Model;
uniform mat4 Projection;

out vec4 color;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    color = color0;
}
