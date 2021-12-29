in vec2 v_position;

out vec4 frag;

uniform vec3 color;
uniform vec2 resolution;
uniform mat4 model;
uniform mat4 view;

vec2 to_view_space(vec2 local) {
    return (view * model * vec4(local, 0.0, 1.0)).xy;
}

void main() {
    vec2 screen_pos = (2.0 * gl_FragCoord.xy - resolution) / resolution.y;
    float delta = fwidth(length(screen_pos));

    vec2 centre = to_view_space(vec2(0.0, 0.0));
    vec2 radius = to_view_space(normalize(v_position)) - centre;
    vec2 centre_to_frag = to_view_space(v_position) - centre;
    float radius_length = length(radius);

    float alpha = smoothstep(
        radius_length, 
        radius_length - 1.5 * delta, 
        length(centre_to_frag)
    );

    frag = vec4(color, alpha);
}