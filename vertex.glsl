#version 150

uniform vec2 windowsize;

in ivec2 position;
in ivec2 texcoord;

out vec2 Texcoord;

void main() {
	// this gives us [0, 1]. We need [-1, 1]. Scale by two, subtract by one in each direction
	vec2 coord = (2.0 * (position / windowsize)) - vec2(1.0);
    gl_Position = vec4(coord, 0.0, 1.0);
    Texcoord = vec2(texcoord);
}
