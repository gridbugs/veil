#version 330 core

// TODO deduplicate this information
const int WIDTH = 20;
const int HEIGHT = 20;
const int NUM_TILE_CHANNELS = 5;

const int TILE_WIDTH = 16 * 2;
const int TILE_HEIGHT = 16 * 2;

const int TEX_WIDTH = 640 * 2;
const int TEX_HEIGHT = 400 * 2;

const float TEX_X_RATIO = float(TILE_WIDTH) / float(TEX_WIDTH);
const float TEX_Y_RATIO = float(TILE_HEIGHT) / float(TEX_HEIGHT);

const int TILE_STATUS_IDX = 3;

// the first NUM_TILE_CHANNELS bits indicate the presence of a channel
const int TILE_STATUS_VISIBLE = 1 << (NUM_TILE_CHANNELS + 0);

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

vec4 resolve_remembered(vec4 data, int status) {
    return vec4(0.0, 0.0, 0.0, 1.0);
}

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

        float x = (float(x_coord) + x_offset) * TEX_X_RATIO;
        float y = (float(y_coord) + y_offset) * TEX_Y_RATIO;

        vec4 colour = texture(t_Texture, vec2(x, y));
        current = blend(current, colour);
    }

    return current;
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
