in vec2 position;

out vec4 v_position;

uniform mat4 model;
uniform mat4 view; 

void main() {
    v_position = vec4(position, 0.0, 1.0);
    gl_Position = view * model * v_position;
}