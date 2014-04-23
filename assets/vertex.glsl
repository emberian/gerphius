#version 150

uniform vec2 windowsize;
uniform float rotation;

in vec2 position;
in vec2 texcoord;

out vec2 Texcoord;

void main() {
    float s = sin(rotation);
    float c = cos(rotation);
    mat4 rzm = mat4(
            c, s, 0, 0,
            -s, c, 0, 0,
            0, 0, 1, 0,
            0, 0, 0, 1
    );
    gl_Position = rzm * vec4(position, 0.0, 1.0);
    Texcoord = vec2(texcoord);
}
