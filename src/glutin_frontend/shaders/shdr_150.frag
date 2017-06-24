#version 150 core

// TODO deduplicate this information
const int WIDTH = 20;
const int HEIGHT = 20;

in vec2 v_TexPos;
in vec2 v_CellPos;

out vec4 Target0;

uniform sampler2D t_Texture;

struct TileMapData {
    vec4 data;
};

uniform b_TileMap {
    TileMapData u_Data[WIDTH * HEIGHT];
};

void main() {

    int x_idx = int(v_CellPos[0]);
    int y_idx = int(v_CellPos[1]);
    int idx = x_idx + y_idx * WIDTH;

    Target0 = texture(t_Texture, v_TexPos);
}
