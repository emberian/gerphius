#version 150

in vec2 Texcoord;

out vec4 out_color;

uniform sampler2D sprite;

void main() {
    vec4 texel = texture(sprite, Texcoord);
    if (texel.a < 0.5)
        discard;

    out_color = vec4(Texcoord, 0, 0.5);
}
