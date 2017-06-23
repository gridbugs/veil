#version 150 core

in vec2 a_Pos;
in vec2 a_TexPos;

out vec2 v_TexPos;

void main() {
    v_TexPos = a_TexPos;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}
