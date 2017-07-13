use gfx;
use gfx::texture::{SamplerInfo, FilterMethod, WrapMode};
use gfx::Factory;
use gfx::traits::FactoryExt;
use gfx_device_gl;
use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};
use genmesh::{Triangulate, Vertices};

use glutin_frontend::formats::ColourFormat;
use glutin_frontend::types::*;

pub type PipelineData = pipe::Data<Resources>;
pub type VertexBufferHandle = gfx::handle::Buffer<Resources, Vertex>;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        tex_pos: [f32; 2] = "a_TexPos",
        tex_pix_pos: [f32; 2] = "a_TexPixPos",
    }

    constant Info {
        tex_size_pix: [f32; 2] = "u_TexSizePix",
        tex_step: [f32; 2] = "u_TexStep",
        tex_half_step: [f32; 2] = "u_TexHalfStep",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        tex_world: gfx::TextureSampler<[f32; 4]> = "t_World",
        tex_overlay: gfx::TextureSampler<[f32; 4]> = "t_Overlay",
        info: gfx::ConstantBuffer<Info> = "b_Info",
        out: gfx::BlendTarget<ColourFormat> =
            ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

pub struct ScalePipeline {
    pub slice: Slice,
    pub state: gfx::PipelineState<Resources, pipe::Meta>,
    pub data: PipelineData,
}

fn create_vertex_buffer(width_px: u32, height_px: u32, factory: &mut gfx_device_gl::Factory) -> (VertexBufferHandle, Slice) {
    let plane = Plane::subdivide(1, 1);

    let vertex_data: Vec<Vertex> = plane.shared_vertex_iter().map(|vertex| {

            let raw_x = vertex.pos[0];
            let raw_y = vertex.pos[1];

            let tex_x = raw_x / 2.0 + 0.5;
            let tex_y = raw_y / 2.0 + 0.5;

            Vertex {
                pos: [raw_x, raw_y],
                tex_pos: [tex_x, tex_y],
                tex_pix_pos: [width_px as f32 * tex_x, height_px as f32 * tex_y],
            }
        })
        .collect();


    let index_data: Vec<u32> = plane.indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|i| i as u32)
        .collect();

    factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..])
}

impl ScalePipeline {
    pub fn new(vertex_shader: &[u8],
               fragment_shader: &[u8],
               view_width_px: u32,
               view_height_px: u32,
               view_world: ShaderResourceView,
               view_overlay: ShaderResourceView,
               rtv: RenderTargetView,
               factory: &mut gfx_device_gl::Factory,
               encoder: &mut Encoder) -> Self {

        let state = factory.create_pipeline_simple(
            vertex_shader,
            fragment_shader,
            pipe::new()
        ).expect("Failed to create pipeline");

        let (vertex_buffer, slice) = create_vertex_buffer(view_width_px, view_height_px, factory);

        let sampler = factory.create_sampler(SamplerInfo::new(FilterMethod::Scale, WrapMode::Tile));

        let data = pipe::Data {
            vbuf: vertex_buffer,
            tex_world: (view_world, sampler.clone()),
            tex_overlay: (view_overlay, sampler),
            info: factory.create_constant_buffer(1),
            out: rtv,
        };

        let info = Info {
            tex_size_pix: [ view_width_px as f32, view_height_px as f32 ],
            tex_step: [ 1.0 / view_width_px as f32, 1.0 / view_height_px as f32 ],
            tex_half_step: [ 0.5 / view_width_px as f32, 0.5 / view_height_px as f32 ],
        };

        encoder.update_buffer(&data.info, &[info], 0).expect("Failed to upload info");

        ScalePipeline {
            slice: slice,
            state: state,
            data: data,
        }
    }

    pub fn draw(&self, encoder: &mut Encoder) {
        encoder.draw(&self.slice, &self.state, &self.data);
    }
}
