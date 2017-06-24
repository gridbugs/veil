use gfx;
use gfx::Device;
use gfx::Factory;
use gfx::traits::FactoryExt;
use gfx_window_glutin;
use glutin;
use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};
use genmesh::{Triangulate, Vertices};
use image;

use tile_buffer::TileBufferCell;

pub type ColourFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const WIDTH_TILES: u32 = 20;
const HEIGHT_TILES: u32 = 20;
const TILE_SIZE: u32 = 32;
const TILE_IDX_BITS: u32 = 5;

const WIDTH_PX: u32 = WIDTH_TILES * TILE_SIZE;
const HEIGHT_PX: u32 = HEIGHT_TILES * TILE_SIZE;
const NUM_TILES: u32 = WIDTH_TILES * HEIGHT_TILES;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        tex_pos: [f32; 2] = "a_TexPos",
        cell_pos: [f32; 2] = "a_CellPos",
    }

    constant TileMapData {
        data: [f32; 4] = "data",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        tile_table: gfx::ConstantBuffer<TileMapData> = "b_TileMap",
        out: gfx::BlendTarget<ColourFormat> =
            ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

impl TileMapData {
    fn new_empty() -> Self {
        TileMapData {
            data: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl From<TileBufferCell> for TileMapData {
    fn from(_cell: TileBufferCell) -> Self {
        unimplemented!()
    }
}

pub fn launch() {

    let builder = glutin::WindowBuilder::new()
        .with_dimensions(WIDTH_PX, HEIGHT_PX)
        .with_title("Veil".to_string());

    let events_loop = glutin::EventsLoop::new();

    let (window, mut device, mut factory, colour_view, _main_depth) =
        gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, &events_loop);

    let img = image::open("resources/tiles.png").expect("failed to open image").to_rgba();
    let (img_width, img_height) = img.dimensions();
    let tex_kind = gfx::texture::Kind::D2(img_width as u16, img_height as u16, gfx::texture::AaMode::Single);
    let (_, texture) = factory.create_texture_immutable_u8::<ColourFormat>(tex_kind, &[&img])
        .expect("Failed to create texture");
    let sampler = factory.create_sampler_linear();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/shdr_150.vert"),
        include_bytes!("shaders/shdr_150.frag"),
        pipe::new()
    ).expect("Failed to create pipeline");

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    const CLEAR_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

    let plane = Plane::subdivide(WIDTH_TILES as usize, HEIGHT_TILES as usize);

    let vertex_data: Vec<Vertex> = plane.shared_vertex_iter().map(|vertex| {

            let raw_x = vertex.pos[0];
            let raw_y = vertex.pos[1];

            let x = raw_x / 2.0 + 0.5;
            let y = 0.5 - raw_y / 2.0;

            Vertex {
                pos: [raw_x, raw_y],
                tex_pos: [x, y],
                cell_pos: [x * WIDTH_TILES as f32, y * HEIGHT_TILES as f32],
            }
        })
        .collect();

    let index_data: Vec<u32> = plane.indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|i| i as u32)
        .collect();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);

    let tile_buffer = factory.create_constant_buffer(NUM_TILES as usize);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        tex: (texture, sampler),
        tile_table: tile_buffer,
        out: colour_view,
    };

    let mut tile_map = Vec::new();
    for _ in 0..NUM_TILES {
        tile_map.push(TileMapData::new_empty());
    }

    'main: loop {
        let mut running = true;
        events_loop.poll_events(|e| {
            let event = if let glutin::Event::WindowEvent { event, .. } = e {
                event
            } else {
                return;
            };
            match event {
                glutin::WindowEvent::Closed => running = false,
                _ => {}
            }
        });

        if !running {
            break;
        }

        encoder.clear(&data.out, CLEAR_COLOR);

        encoder.update_buffer(&data.tile_table, &tile_map, 0)
            .expect("Failed to update tile buffer");

        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().expect("Failed to swap buffers");
        device.cleanup();
    }
}
