in vec4 v_position;

out vec4 frag;

uniform vec3 color;

void main() {
    const vec4 zero = vec4(0.0, 0.0, 0.0, 1.0);
    if (distance(zero, v_position) > 1.0) {
        discard;
    }
    frag = vec4(color, 1.0);
}