#version 150

uniform vec2 windowsize;
uniform float rotation;
uniform vec2 center;

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

    vec4 pos = vec4(position - center, 0.0, 1.0);

    gl_Position = (rzm * pos) + vec4(center, 0.0, 0.0);
    Texcoord = vec2(texcoord);
}
