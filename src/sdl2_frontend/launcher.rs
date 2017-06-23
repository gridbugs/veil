use sdl2;
use sdl2::image::INIT_PNG;
use sdl2_frontend::renderer::*;
use sdl2_frontend::renderer_env::*;
use sdl2_frontend::input::*;
use frame::AnimationMode;
use launch;

const WIDTH_TILES: u32 = 20;
const HEIGHT_TILES: u32 = 20;
const TILE_SIZE: u32 = 32;

const WIDTH_PX: u32 = WIDTH_TILES * TILE_SIZE;
const HEIGHT_PX: u32 = HEIGHT_TILES * TILE_SIZE;

pub fn launch() {

    let sdl = sdl2::init().expect("SDL2 initialization failed");
    let video = sdl.video().expect("Failed to connect to video subsystem");
    let mut renderer_env = RendererEnv::new(WIDTH_PX, HEIGHT_PX, &video);
    sdl2::image::init(INIT_PNG).expect("Failed to connect to image subsystem");

    let mut renderer = SdlGameRenderer::new(WIDTH_TILES as usize,
                                            HEIGHT_TILES as usize,
                                            &mut renderer_env,
                                            "resources/tiles.png",
                                            "resources/tiles.toml");
    let event_pump = sdl.event_pump().expect("Failed to initialize event pump");
    let mut input = SdlGameInput::new(event_pump, 60, AnimationMode::RealTime);

    launch::launch(&mut renderer, &mut input);
}
