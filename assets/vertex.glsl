#version 150

uniform vec2 windowsize;

in vec2 position;
in vec2 texcoord;

out vec2 Texcoord;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    Texcoord = vec2(texcoord);
}
