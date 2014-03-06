#version 140

uniform vec2 windowsize;

in ivec2 position;
in vec2 texcoord;

out vec2 Texcoord;

void main() {
    gl_Position = vec4(position / windowsize, 0.0, 1.0);
    Texcoord = texcoord;
}
