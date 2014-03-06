in vec2 Texcoord;

out vec4 out_color;

uniform sampler2D sprite;

void main() {
	out_color = texture(sprite, Texcoord);
}
