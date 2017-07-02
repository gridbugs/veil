use gfx;
use gfx::Device;
use gfx::Factory;
use gfx::traits::FactoryExt;
use gfx_window_glutin;
use glutin;
use gfx_device_gl;
use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};
use genmesh::{Triangulate, Vertices};
use image;

use resources::{self, TILE_SHEET_SPEC, TILE_SHEET_IMAGE};
use tile_buffer::TileBufferCell;
use simple_file;
use tile_desc::TileDesc;

use cgmath::Vector2;
use knowledge::PlayerKnowledgeGrid;
use renderer::{GameRenderer, GameRendererConfig};
use render_overlay::RenderOverlay;
use input::{GameInput, InputEvent, ExternalEvent};
use frame::Frame;
use common_input::CommonInput;
use tile_buffer::TileBuffer;
use tile::{TileResolver, NUM_TILE_CHANNELS};

type ColourFormat = gfx::format::Srgba8;
type DepthFormat = gfx::format::DepthStencil;

const FPS: u32 = 60;

const WIDTH_TILES: u32 = 30;
const HEIGHT_TILES: u32 = 30;
const TILE_SIZE: u32 = 24;
const TILE_IDX_BITS: u32 = 5;

const WIDTH_PX: u32 = WIDTH_TILES * TILE_SIZE;
const HEIGHT_PX: u32 = HEIGHT_TILES * TILE_SIZE;
const NUM_TILES: u32 = WIDTH_TILES * HEIGHT_TILES;

const CLEAR_COLOUR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        tex_pos: [f32; 2] = "a_TexPos",
        cell_pos: [f32; 2] = "a_CellPos",
    }

    constant TileMapData {
        data: [f32; 4] = "data",
    }

    constant TileMapInfo {
        ratio: [f32; 2] = "u_TexRatio",
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

const TILE_STATUS_IDX: usize = 3;
const TILE_STATUS_VISIBLE: u32 = 1 << (NUM_TILE_CHANNELS + 0);

impl TileMapData {
    fn new_empty() -> Self {
        let status: f32 = f32::from_bits((1<<5) | 1);
        let first: f32 = f32::from_bits((2 << 8) | 0);
        TileMapData {
            data: [first, 0.0, 0.0, status],
        }
    }

    fn channel_mask(channel_idx: usize) -> u32 {
        if channel_idx % 2 == 0 {
            0xffff0000
        } else {
            0x0000ffff
        }
    }

    fn channel_shift(channel_idx: usize) -> u32 {
        (channel_idx as u32 % 2) * 16
    }

    fn set_visible(&mut self) {
        let mut current = self.data[TILE_STATUS_IDX].to_bits();
        current |= TILE_STATUS_VISIBLE;
        self.data[TILE_STATUS_IDX] = f32::from_bits(current);
    }

    fn clear_visible(&mut self) {
        let mut current = self.data[TILE_STATUS_IDX].to_bits();
        current &= !TILE_STATUS_VISIBLE;
        self.data[TILE_STATUS_IDX] = f32::from_bits(current);
    }

    fn set_channel_present(&mut self, channel_idx: usize) {
        let mut current = self.data[TILE_STATUS_IDX].to_bits();
        current |= 1 << (channel_idx as u32);
        self.data[TILE_STATUS_IDX] = f32::from_bits(current);
    }

    fn clear_channel_present(&mut self, channel_idx: usize) {
        let mut current = self.data[TILE_STATUS_IDX].to_bits();
        current &= !(1 << (channel_idx as u32));
        self.data[TILE_STATUS_IDX] = f32::from_bits(current);
    }

    fn set_channel(&mut self, channel_idx: usize, x: u8, y: u8) {
        let idx = channel_idx / 2;
        let current = self.data[idx].to_bits();
        let masked = current & Self::channel_mask(channel_idx);
        let result = masked | ((x as u32 | (y as u32) << 8) << Self::channel_shift(channel_idx));
        self.data[idx] = f32::from_bits(result);
    }

    fn set_from_tile_buffer_cell(&mut self, cell: &TileBufferCell) {
        for channel_idx in 0..NUM_TILE_CHANNELS {
            if let Some(coord) = cell.channels[channel_idx] {
                self.set_channel(channel_idx, coord.x as u8, coord.y as u8);
                self.set_channel_present(channel_idx);
            } else {
                self.clear_channel_present(channel_idx);
            }
            if cell.visible {
                self.set_visible();
            } else {
                self.clear_visible();
            }
        }
    }
}

impl TileMapInfo {
    fn new(tile_size: u32, tex_width: u32, tex_height: u32) -> Self {
        TileMapInfo {
            ratio: [
                tile_size as f32 / tex_width as f32,
                tile_size as f32 / tex_height as f32,
            ]
        }
    }
}

pub struct GlutinGameRenderer {
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    window: glutin::Window,
    device: gfx_device_gl::Device,
    slice: gfx::Slice<gfx_device_gl::Resources>,
    pso: gfx::PipelineState<gfx_device_gl::Resources, pipe::Meta>,
    pipeline_data: pipe::Data<gfx_device_gl::Resources>,
    tile_map: Vec<TileMapData>,
    tile_buffer: TileBuffer,
    tile_resolver: TileResolver,
    player_coord: Vector2<i32>,
    offset_delta: Vector2<i32>,
    config: GameRendererConfig,
}

pub struct GlutinGameInput {
    events_loop: glutin::EventsLoop,
    common_input: CommonInput,
}

pub fn create() -> (GlutinGameRenderer, GlutinGameInput) {

    let tile_path = resources::resource_path(TILE_SHEET_IMAGE);
    let img = image::open(tile_path).expect("failed to open image").to_rgba();
    let (img_width, img_height) = img.dimensions();

    let builder = glutin::WindowBuilder::new()
        .with_decorations(true)
        .with_dimensions(WIDTH_PX, HEIGHT_PX)
        .with_min_dimensions(WIDTH_PX, HEIGHT_PX)
        .with_max_dimensions(WIDTH_PX, HEIGHT_PX)
        .with_title("Veil".to_string());

    let events_loop = glutin::EventsLoop::new();

    let (window, device, mut factory, colour_view, _main_depth) =
        gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, &events_loop);

    let tex_kind = gfx::texture::Kind::D2(img_width as u16, img_height as u16, gfx::texture::AaMode::Single);
    let (_, texture) = factory.create_texture_immutable_u8::<ColourFormat>(tex_kind, &[&img])
        .expect("Failed to create texture");
    let sampler = factory.create_sampler_linear();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shaders/shdr_330.vert"),
        include_bytes!("shaders/shdr_330.frag"),
        pipe::new()
    ).expect("Failed to create pipeline");

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let plane = Plane::subdivide(1, 1);

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
    let tile_map_info = factory.create_constant_buffer(1);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        tex: (texture, sampler),
        tile_map_info: tile_map_info,
        tile_table: tile_buffer,
        out: colour_view,
    };

    let tile_desc: TileDesc = simple_file::read_toml(&resources::resource_path(TILE_SHEET_SPEC))
        .expect("Failed to read tile spec");

    encoder.update_buffer(&data.tile_map_info, &[TileMapInfo::new(tile_desc.tile_size_scaled(), img_width, img_height)], 0)
        .expect("Failed to update texture ratio");

    let mut tile_map = Vec::new();
    for _ in 0..NUM_TILES {
        tile_map.push(TileMapData::new_empty());
    }

    let renderer = GlutinGameRenderer {
        encoder: encoder,
        window: window,
        device: device,
        slice: slice,
        pso: pso,
        pipeline_data: data,
        tile_map: tile_map,
        tile_buffer: TileBuffer::new(WIDTH_TILES as usize, HEIGHT_TILES as usize),
        tile_resolver: TileResolver::from_desc(&tile_desc),
        player_coord: Vector2::new(0, 0),
        offset_delta: Vector2::new(WIDTH_TILES as i32 / 2, HEIGHT_TILES as i32 / 2),
        config: Default::default(),
    };

    let input = GlutinGameInput {
        events_loop: events_loop,
        common_input: CommonInput::from_fps(FPS),
    };

    (renderer, input)
}

pub fn example(renderer: &mut GlutinGameRenderer, input: &mut GlutinGameInput) {
    'main: loop {
        let mut running = true;
        input.events_loop.poll_events(|e| {
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

        renderer.encoder.clear(&renderer.pipeline_data.out, CLEAR_COLOUR);

        renderer.encoder.update_buffer(&renderer.pipeline_data.tile_table, &renderer.tile_map, 0)
            .expect("Failed to update tile buffer");

        renderer.encoder.draw(&renderer.slice, &renderer.pso, &renderer.pipeline_data);
        renderer.encoder.flush(&mut renderer.device);
        renderer.window.swap_buffers().expect("Failed to swap buffers");
        renderer.device.cleanup();
    }

}

impl GameRenderer for GlutinGameRenderer {
    fn clear(&mut self) {
        self.encoder.clear(&self.pipeline_data.out, CLEAR_COLOUR);
    }

    fn update_player_position(&mut self, player_coord: Vector2<i32>) {
        self.player_coord = player_coord;
    }

    fn update_player_knowledge(&mut self, knowledge: &PlayerKnowledgeGrid, time: u64) {
        self.tile_buffer.update(self.player_coord - self.offset_delta, knowledge, &self.tile_resolver, time);

        for (idx, cell) in self.tile_buffer.iter().enumerate() {
            self.tile_map[idx].set_from_tile_buffer_cell(cell);
        }

        self.encoder.update_buffer(&self.pipeline_data.tile_table, &self.tile_map, 0)
            .expect("Failed to update buffer");
    }

    fn draw(&mut self) {
        self.encoder.draw(&self.slice, &self.pso, &self.pipeline_data);
    }

    fn draw_overlay(&mut self, overlay: RenderOverlay) {

    }

    fn publish(&mut self) {
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().expect("Failed to swap buffers");
        self.device.cleanup();
    }

    fn set_config(&mut self, config: GameRendererConfig) {
        self.config = config;
    }

    fn config(&self) -> GameRendererConfig {
        self.config
    }
}

impl GameInput for GlutinGameInput {
    fn next_input(&mut self) -> InputEvent {
        loop {
            if let Some(input_event) = self.next_external().input() {
                return input_event;
            }
        }
    }

    fn next_frame(&mut self) -> Frame {
        let frame = self.common_input.wait_for_next_frame();

        self.events_loop.poll_events(|_| {});

        frame
    }

    fn next_external(&mut self) -> ExternalEvent {
        let frame = self.common_input.wait_for_next_frame();

        let mut input_event = None;

        self.events_loop.poll_events(|e| {
            let event = if let glutin::Event::WindowEvent { event, .. } = e {
                event
            } else {
                return;
            };

            match event {
                glutin::WindowEvent::Closed => input_event = Some(InputEvent::Quit),
                _ => {}
            }
        });

        if let Some(input_event) = input_event {
            return ExternalEvent::new(input_event, frame);
        }

        ExternalEvent::with_frame(frame)
    }
}
