use std::collections::BTreeMap;

use gfx;
use gfx::Factory;
use gfx::traits::FactoryExt;
use gfx::texture::{SamplerInfo, FilterMethod, WrapMode};
use gfx_device_gl;
use image::{self, RgbaImage};
use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};
use genmesh::{Triangulate, Vertices};
use handlebars::Handlebars;

use glutin_frontend::formats::ColourFormat;
use resources::{self, TILE_SHEET_SPEC, TILE_SHEET_IMAGE};
use simple_file;
use tile_desc::TileDesc;

type VertexBufferHandle = gfx::handle::Buffer<gfx_device_gl::Resources, Vertex>;
type Slice = gfx::Slice<gfx_device_gl::Resources>;
type PipelineData = pipe::Data<gfx_device_gl::Resources>;
pub type RenderTargetView = gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColourFormat>;
pub type Encoder = gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>;

pub struct TileMapPipeline {
    pub slice: Slice,
    pub state: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    pub data: PipelineData,
    pub buffer: Vec<TileMapData>,
    pub image: RgbaImage,
    pub description: TileDesc,
}

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        cell_pos: [f32; 2] = "a_CellPos",
    }

    constant TileMapData {
        data: [f32; 4] = "data",
    }

    constant TileMapInfo {
        ratio: [f32; 2] = "u_TexRatio",
        centre: [f32; 2] = "u_Centre",
        tile_sheet_size_pix: [f32; 2] = "u_TileSheetSizePix",
        tile_size_pix: f32 = "u_TileSizePix",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        tile_table: gfx::ConstantBuffer<TileMapData> = "b_TileMap",
        tile_map_info: gfx::ConstantBuffer<TileMapInfo> = "b_TileMapInfo",
        out: gfx::BlendTarget<ColourFormat> =
            ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

impl Default for TileMapData {
    fn default() -> Self {
        TileMapData {
            data: [0.0; 4],
        }
    }
}

pub trait UpdateTileMapData {
    fn update(&self, idx: usize, data: &mut [TileMapData]);
}

impl TileMapInfo {
    pub fn new(tile_size: u32, tex_width: u32, tex_height: u32, width: u32, height: u32) -> Self {
        TileMapInfo {
            ratio: [
                tile_size as f32 / tex_width as f32,
                tile_size as f32 / tex_height as f32,
            ],
            centre: [
                width as f32 / 2.0 + 0.5,
                height as f32 / 2.0 + 0.5,
            ],
            tile_size_pix: tile_size as f32,
            tile_sheet_size_pix: [
                tex_width as f32,
                tex_height as f32,
            ],
        }
    }
}

fn read_tiles() -> (RgbaImage, TileDesc) {
    let tile_path = resources::res_path(TILE_SHEET_IMAGE);
    let img = image::open(tile_path).expect("failed to open image").to_rgba();

    let tile_desc: TileDesc = simple_file::read_toml(&resources::res_path(TILE_SHEET_SPEC))
        .expect("Failed to read tile spec");

    (img, tile_desc)
}

type Texture = gfx::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>;

fn create_texture(img: &RgbaImage, factory: &mut gfx_device_gl::Factory) -> Texture {

    let (img_width, img_height) = img.dimensions();

    let tex_kind = gfx::texture::Kind::D2(img_width as u16, img_height as u16, gfx::texture::AaMode::Single);
    let (_, texture) = factory.create_texture_immutable_u8::<ColourFormat>(tex_kind, &[&img])
        .expect("Failed to create texture");

    texture
}

fn create_buffer(num_tiles: usize) -> Vec<TileMapData> {
    let mut buffer = Vec::new();
    for _ in 0..num_tiles {
        buffer.push(TileMapData::default());
    }

    buffer
}

fn create_vertex_buffer(width: u32, height: u32, factory: &mut gfx_device_gl::Factory) -> (VertexBufferHandle, Slice) {
    let plane = Plane::subdivide(1, 1);

    let vertex_data: Vec<Vertex> = plane.shared_vertex_iter().map(|vertex| {

            let raw_x = vertex.pos[0];
            let raw_y = vertex.pos[1];

            let x = raw_x / 2.0 + 0.5;
            let y = 0.5 - raw_y / 2.0;

            Vertex {
                pos: [raw_x, raw_y],
                cell_pos: [x * width as f32, y * height as f32],

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

fn create_pipeline_data(width: u32, height: u32,
                        rtv: RenderTargetView,
                        vertex_buffer: VertexBufferHandle,
                        texture: Texture,
                        factory: &mut gfx_device_gl::Factory) -> PipelineData {

//    let sampler = factory.create_sampler_linear();
    let sampler = factory.create_sampler(SamplerInfo::new(FilterMethod::Scale, WrapMode::Tile));


    let tile_table = factory.create_constant_buffer((width * height) as usize);
    let tile_map_info = factory.create_constant_buffer(1);

    pipe::Data {
        vbuf: vertex_buffer,
        tex: (texture, sampler),
        tile_map_info: tile_map_info,
        tile_table: tile_table,
        out: rtv,
    }
}

fn update_tile_map_info(tile_map_info: &gfx::handle::Buffer<gfx_device_gl::Resources, TileMapInfo>,
                        tile_img: &RgbaImage,
                        tile_desc: &TileDesc,
                        width: u32,
                        height: u32,
                        encoder: &mut Encoder) {

    let (img_width, img_height) = tile_img.dimensions();
    let info = TileMapInfo::new(tile_desc.tile_size_scaled(), img_width, img_height, width, height);
    encoder.update_buffer(tile_map_info,
                          &[info],
                          0).expect("Failed to upload tile map info");
}

pub type ShaderTemplateInfo<'a> = BTreeMap<&'a str, u32>;

fn instantiate_shader_template(handlebars: &Handlebars,
                               shader: &[u8],
                               internal_template_info: &ShaderTemplateInfo,
                               external_template_info: &ShaderTemplateInfo) -> String {

    let shader_str = ::std::str::from_utf8(shader).expect("Failed to convert shader to utf8");

    let template_info: BTreeMap<_, _> = internal_template_info.iter()
        .chain(external_template_info.iter())
        .collect();

    handlebars.template_render(shader_str.as_ref(), &template_info)
        .expect("Failed to instantiate shader template with external info")
}


impl TileMapPipeline {
    pub fn new(width: u32, height: u32,
               tile_data_size: usize,
               vertex_shader: &[u8],
               fragment_shader: &[u8],
               external_shader_template_info: ShaderTemplateInfo,
               rtv: RenderTargetView,
               factory: &mut gfx_device_gl::Factory,
               encoder: &mut Encoder) -> Self {

        let (tile_img, tile_desc) = read_tiles();

        let texture = create_texture(&tile_img, factory);

        let mut handlebars = Handlebars::new();
        // prevent xml escaping
        handlebars.register_escape_fn(|input| input.to_string());

        let internal_shader_template_info = btreemap!{
            "WIDTH_TILES" => width,
            "HEIGHT_TILES" => height,
        };

        let state = factory.create_pipeline_simple(
            &instantiate_shader_template(&handlebars, vertex_shader,
                                         &internal_shader_template_info,
                                         &external_shader_template_info).into_bytes(),
            &instantiate_shader_template(&handlebars, fragment_shader,
                                         &internal_shader_template_info,
                                         &external_shader_template_info).into_bytes(),
            pipe::new()
        ).expect("Failed to create pipeline");

        let (vertex_buffer, slice) = create_vertex_buffer(width, height, factory);

        let pipeline_data = create_pipeline_data(width, height, rtv, vertex_buffer, texture, factory);
        update_tile_map_info(&pipeline_data.tile_map_info, &tile_img, &tile_desc, width, height, encoder);

        let buffer = create_buffer(tile_data_size);

        TileMapPipeline {
            slice: slice,
            state: state,
            data: pipeline_data,
            buffer: buffer,
            image: tile_img,
            description: tile_desc,
        }
    }

    pub fn update_tile_map_data_cell<U: UpdateTileMapData>(&mut self, idx: usize, u: &U) {
        u.update(idx, &mut self.buffer);
    }

    pub fn update_tile_map_data<I, U>(&mut self, i: I)
        where U: UpdateTileMapData,
              I: Iterator<Item=U>,
    {
        for (idx, u) in i.enumerate() {
            u.update(idx, &mut self.buffer);
        }
    }

    pub fn update_buffer(&self, encoder: &mut Encoder) {
        encoder.update_buffer(&self.data.tile_table, &self.buffer, 0)
            .expect("Failed to update buffer");
    }

    pub fn draw(&self, encoder: &mut Encoder) {
        encoder.draw(&self.slice, &self.state, &self.data);
    }
}
