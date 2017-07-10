#version 330 core

// general parameters
const int WIDTH = {{WIDTH_TILES}};
const int HEIGHT = {{HEIGHT_TILES}};

// shader-specific parameters
const int TILE_STATUS_IDX = {{TILE_STATUS_IDX}};
const int TILE_STATUS_ENABLED = {{TILE_STATUS_ENABLED}};

in vec2 v_CellPos;

out vec4 Target0;

uniform sampler2D t_Texture;

uniform b_TileMapInfo {
    vec2 u_TexRatio;
};

struct TileMapData {
    vec4 data;
};

uniform b_TileMap {
    TileMapData u_Data[WIDTH * HEIGHT];
};

vec4 blend(vec4 current, vec4 new) {
    vec3 delta = vec3(new - current);
    vec3 result = vec3(current) + delta * new[3];
    return vec4(result, max(current[3], new[3]));
}

vec4 resolve(vec4 data, int status) {
    vec4 current = vec4(0.0, 0.0, 0.0, 0.0);
    float x_offset = fract(v_CellPos[0]);
    float y_offset = fract(v_CellPos[1]);

    int word = floatBitsToInt(data[0]);

    int x_coord = word & 0xff;
    int y_coord = (word >> 8) & 0xff;

    float x = (float(x_coord) + x_offset) * u_TexRatio[0];
    float y = (float(y_coord) + y_offset) * u_TexRatio[1];

    vec4 colour = texture(t_Texture, vec2(x, y));
    current = blend(current, colour);

    return current;
}

void main() {

    int x_idx = int(v_CellPos[0]);
    int y_idx = int(v_CellPos[1]);
    int tile_map_idx = x_idx + y_idx * WIDTH;

    vec4 cell_info = u_Data[tile_map_idx].data;

    int status = floatBitsToInt(cell_info[TILE_STATUS_IDX]);

    if ((status & TILE_STATUS_ENABLED) != 0) {
        Target0 = resolve(cell_info, status);
    } else {
        Target0 = vec4(0.0, 0.0, 0.0, 0.0);
    }
}
