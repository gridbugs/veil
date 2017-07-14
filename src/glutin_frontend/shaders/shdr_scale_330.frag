#version 330 core

out vec4 Target0;

uniform sampler2D t_World;
uniform sampler2D t_Overlay;

in vec2 v_TexPos;
in vec2 v_TexPixPos;

uniform b_Info {
    vec2 u_TexSizePix;
    vec2 u_TexStep;
    vec2 u_TexHalfStep;
};

vec4 over_blend(vec4 new, vec4 existing) {
    return vec4(new.a) * new + vec4(1.0 - new.a) * existing;
}

vec2 from_pix_centre(vec2 coord) {
    return fract(coord) - vec2(0.5);
}

const float THRESHOLD = 1.0 / 12.0;
const float THRESHOLD_FROM_CENTRE = 0.5 - THRESHOLD;
const float THRESHOLD_X_2 = THRESHOLD * 2;

vec4 sample_textures(vec2 coord) {
    vec4 world = texture(t_World, coord);
    vec4 overlay = texture(t_Overlay, coord);
    return over_blend(overlay, world);
}

void main() {

    // TODO: work out why the small offset is required
    vec2 orig_tex_coord = v_TexPos + u_TexStep * 0.001;

    vec2 src_pix_coord = orig_tex_coord * u_TexSizePix;
    vec2 fpc = from_pix_centre(src_pix_coord);

    vec4 main_col = sample_textures(orig_tex_coord);

    vec4 x_col = main_col;
    if (fpc.x > THRESHOLD_FROM_CENTRE) {
        vec2 stepped = orig_tex_coord + vec2(u_TexHalfStep.x, 0.0);
        if (stepped.x <= 1.0) {
            vec4 stepped_col = sample_textures(stepped);
            float weight = (fpc.x - THRESHOLD_FROM_CENTRE) / THRESHOLD_X_2;
            x_col = weight * main_col + (1.0 - weight) * stepped_col;
        }
    } else if (fpc.x < -THRESHOLD_FROM_CENTRE) {
        vec2 stepped = orig_tex_coord - vec2(u_TexHalfStep.x, 0.0);
        if (stepped.x >= 0) {
            vec4 stepped_col = sample_textures(stepped);
            float weight = -(fpc.x + THRESHOLD_FROM_CENTRE) / THRESHOLD_X_2;
            x_col = weight * main_col + (1.0 - weight) * stepped_col;
        }
    }

    vec4 y_col = main_col;
    if (fpc.y > THRESHOLD_FROM_CENTRE) {
        vec2 stepped = orig_tex_coord + vec2(0.0, u_TexHalfStep.y);
        if (stepped.y <= 1.0) {
            vec4 stepped_col = sample_textures(stepped);
            float weight = (fpc.y - THRESHOLD_FROM_CENTRE) / THRESHOLD_X_2;
            y_col = weight * main_col + (1.0 - weight) * stepped_col;
        }
    } else if (fpc.y < -THRESHOLD_FROM_CENTRE) {
        vec2 stepped = orig_tex_coord - vec2(0.0, u_TexHalfStep.y);
        if (stepped.y >= 0) {
            vec4 stepped_col = sample_textures(stepped);
            float weight = -(fpc.y + THRESHOLD_FROM_CENTRE) / THRESHOLD_X_2;
            y_col = weight * main_col + (1.0 - weight) * stepped_col;
        }
    }

    Target0 = (x_col + y_col) / 2.0;
}
