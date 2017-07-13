use gfx;
use gfx::Device;
use gfx_window_glutin;
use glutin;
use gfx_device_gl;

use glutin_frontend::input;
use glutin_frontend::world_tile;
use glutin_frontend::overlay_tile::{self, OverlayCoord};
use glutin_frontend::formats::{ColourFormat, DepthFormat};
use glutin_frontend::tile_map::{TileMapPipeline, UpdateTileMapData};
use glutin_frontend::sizes::{WIDTH_TILES, HEIGHT_TILES, TILE_SIZE};

use cgmath::Vector2;

use knowledge::PlayerKnowledgeGrid;
use renderer::{GameRenderer, GameRendererConfig};
use render_overlay::RenderOverlay;
use input::{GameInput, InputEvent, ExternalEvent};
use frame::Frame;
use common_input::CommonInput;
use tile_buffer::TileBuffer;
use tile::TileResolver;
use grid::static_grid::StaticGridIdx;
use content::OverlayType;

const FPS: u32 = 60;

const WIDTH_PX: u32 = WIDTH_TILES * TILE_SIZE;
const HEIGHT_PX: u32 = HEIGHT_TILES * TILE_SIZE;
const NUM_TILES: u32 = WIDTH_TILES * HEIGHT_TILES;

const CLEAR_COLOUR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct GlutinGameRenderer {
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    window: glutin::Window,
    device: gfx_device_gl::Device,
    world_pipeline: TileMapPipeline,
    overlay_pipeline: TileMapPipeline,
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

    let builder = glutin::WindowBuilder::new()
        .with_decorations(true)
        .with_dimensions(WIDTH_PX, HEIGHT_PX)
        .with_min_dimensions(WIDTH_PX, HEIGHT_PX)
        .with_max_dimensions(WIDTH_PX, HEIGHT_PX)
        .with_title("Veil".to_string());

    let events_loop = glutin::EventsLoop::new();

    let (window, device, mut factory, rtv, _) =
        gfx_window_glutin::init::<ColourFormat, DepthFormat>(builder, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut world_pipeline = TileMapPipeline::new(WIDTH_TILES, HEIGHT_TILES, (WIDTH_TILES * HEIGHT_TILES) as usize,
                                                  include_bytes!("shaders/shdr_330.vert"),
                                                  include_bytes!("shaders/shdr_world_330.frag"),
                                                  world_tile::shader_template_info(),
                                                  rtv.clone(), &mut factory, &mut encoder);

    world_tile::init_tile_map_data(&mut world_pipeline.buffer);

    let overlay_pipeline = TileMapPipeline::new(WIDTH_TILES, HEIGHT_TILES, (WIDTH_TILES * HEIGHT_TILES) as usize,
                                                include_bytes!("shaders/shdr_330.vert"),
                                                include_bytes!("shaders/shdr_overlay_330.frag"),
                                                overlay_tile::shader_template_info(),
                                                rtv, &mut factory, &mut encoder);

    let renderer = GlutinGameRenderer {
        encoder: encoder,
        window: window,
        device: device,
        tile_resolver: TileResolver::from_desc(&world_pipeline.description),
        world_pipeline: world_pipeline,
        overlay_pipeline: overlay_pipeline,
        tile_buffer: TileBuffer::new(WIDTH_TILES as usize, HEIGHT_TILES as usize),
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

impl GameRenderer for GlutinGameRenderer {
    fn clear(&mut self) {
        self.encoder.clear(&self.world_pipeline.data.out, CLEAR_COLOUR);
    }

    fn update_player_position(&mut self, player_coord: Vector2<i32>) {
        self.player_coord = player_coord;
    }

    fn update_player_knowledge(&mut self, knowledge: &PlayerKnowledgeGrid, time: u64) {
        self.tile_buffer.update(self.player_coord - self.offset_delta, knowledge, &self.tile_resolver, time);

        self.world_pipeline.update_tile_map_data(self.tile_buffer.iter());

        self.world_pipeline.update_buffer(&mut self.encoder);
    }

    fn draw(&mut self) {
        self.world_pipeline.draw(&mut self.encoder);
    }

    fn draw_overlay(&mut self, overlay: RenderOverlay) {

        let offset = self.player_coord - self.offset_delta;

        let mid_tile = self.tile_resolver.resolve_overlay(OverlayType::AimLineMid);
        let end_tile = self.tile_resolver.resolve_overlay(OverlayType::AimLineEnd);

        let (mut traverse, end) = overlay.aim_line.split_end();

        let offset_end = end - offset;
        let wrap_width = self.tile_buffer.width();
        let tile_map_idx = offset_end.wrap_to_index(wrap_width);

        overlay_tile::clear_tile_map_data(&mut self.overlay_pipeline.buffer);
        OverlayCoord(end_tile).update(tile_map_idx, &mut self.overlay_pipeline.buffer);

        // skip the start
        traverse.step_in_place();

        for coord in traverse {
            let offset_coord = coord - offset;
            let tile_map_idx = offset_coord.wrap_to_index(wrap_width);
            OverlayCoord(mid_tile).update(tile_map_idx, &mut self.overlay_pipeline.buffer);
        }


        self.encoder.update_buffer(&self.overlay_pipeline.data.tile_table, &self.overlay_pipeline.buffer, 0)
            .expect("Failed to update buffer");

        self.encoder.draw(&self.overlay_pipeline.slice, &self.overlay_pipeline.state, &self.overlay_pipeline.data);
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
            if let Some(event) = input::convert_event(e) {
                input_event = Some(event);
            }
        });

        if let Some(input_event) = input_event {
            return ExternalEvent::new(input_event, frame);
        }

        ExternalEvent::with_frame(frame)
    }
}
