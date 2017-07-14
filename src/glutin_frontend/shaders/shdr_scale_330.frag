#version 330 core

out vec4 Target0;

uniform sampler2D t_World;
uniform sampler2D t_Overlay;

in vec2 v_TexPos;

vec4 over_blend(vec4 new, vec4 existing) {
    return vec4(new.a) * new + vec4(1.0 - new.a) * existing;
}

void main() {
    vec4 world = texture(t_World, v_TexPos);
    vec4 overlay = texture(t_Overlay, v_TexPos);
    Target0 = over_blend(overlay, world);
}
