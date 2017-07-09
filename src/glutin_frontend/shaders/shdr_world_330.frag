#version 330 core

// general parameters
const int WIDTH = {{WIDTH_TILES}};
const int HEIGHT = {{HEIGHT_TILES}};

// shader-specific parameters
const int NUM_TILE_CHANNELS = {{NUM_TILE_CHANNELS}};
const int TILE_STATUS_IDX = {{TILE_STATUS_IDX}};
const int TILE_STATUS_VISIBLE = {{TILE_STATUS_VISIBLE}};

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

vec4 resolve_visible(vec4 data, int status) {
    vec4 current = vec4(0.0, 0.0, 0.0, 0.0);
    float x_offset = fract(v_CellPos[0]);
    float y_offset = fract(v_CellPos[1]);

    for (int i = 0; i < NUM_TILE_CHANNELS; i++) {
        if ((status & (1 << i)) == 0) {
            continue;
        }

        int word = floatBitsToInt(data[i / 2]);
        if ((i % 2) == 1) {
            word >>= 16;
        }

        int x_coord = word & 0xff;
        int y_coord = (word >> 8) & 0xff;

        float x = (float(x_coord) + x_offset) * u_TexRatio[0];
        float y = (float(y_coord) + y_offset) * u_TexRatio[1];

        vec4 colour = texture(t_Texture, vec2(x, y));
        current = blend(current, colour);
    }

    return current;
}

vec4 resolve_remembered(vec4 data, int status) {
    // TODO
    return vec4(0.0, 0.0, 0.0, 1.0);
}

void main() {

    int x_idx = int(v_CellPos[0]);
    int y_idx = int(v_CellPos[1]);
    int tile_map_idx = x_idx + y_idx * WIDTH;

    vec4 cell_info = u_Data[tile_map_idx].data;

    int status = floatBitsToInt(cell_info[TILE_STATUS_IDX]);

    if ((status & TILE_STATUS_VISIBLE) != 0) {
        Target0 = resolve_visible(cell_info, status);
    } else {
        Target0 = resolve_remembered(cell_info, status);
    }
}
