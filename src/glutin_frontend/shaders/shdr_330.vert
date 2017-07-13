#version 330 core

in vec2 a_Pos;
in vec2 a_CellPos;

out vec2 v_CellPos;

void main() {
    v_CellPos = a_CellPos;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}
