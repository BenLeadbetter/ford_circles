in vec2 position;
out vec2 v_position;

uniform mat4 model;
uniform mat4 view;

void main() {   
    gl_Position = view * model * vec4(position, 0.0, 1.0);
    v_position = position;
}