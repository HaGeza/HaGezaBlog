#version 330
precision highp float;

in vec4 color;
in vec3 normal3;

uniform vec3 light_direction;

out vec4 final_color;

void main() {
    float diffuse = max(dot(normalize(normal3), normalize(light_direction)), 0.001);
    final_color = color * diffuse;
}
