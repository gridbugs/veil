use sdl2;
use sdl2::image::INIT_PNG;
use sdl2::event::Event;
use cgmath::Vector2;
use sdl2_frontend::renderer::*;
use sdl2_frontend::renderer_env::*;
use entity_store::*;
use spatial_hash::*;
use content::prototypes;
use entity_id_allocator::*;
use knowledge::*;
use observation::*;

const WIDTH: usize = 12;
const HEIGHT: usize = 6;

const WIDTH_PX: u32 = 800;
const HEIGHT_PX: u32 = 600;

pub fn launch() {

    let level_str = vec![
        "############",
        "#..#....#..#",
        "#..#....#..#",
        "#.@...###..#",
        "#..........#",
        "############",
    ];

    let mut entity_store = EntityStore::new();
    let mut change = EntityStoreChange::new();
    let mut allocator = EntityIdAllocator::new();
    let mut spatial_hash = SpatialHashTable::new(WIDTH, HEIGHT);

    let mut pc = 0;
    let mut y = 0;
    for row in level_str.iter() {
        let mut x = 0;
        for ch in row.chars() {
            match ch {
                '#' => {
                    prototypes::wall(&mut change, allocator.allocate(), Vector2::new(x, y));
                    prototypes::stone_floor(&mut change, allocator.allocate(), Vector2::new(x, y));
                }
                '.' => {
                    prototypes::stone_floor(&mut change, allocator.allocate(), Vector2::new(x, y));
                }
                '@' => {
                    pc = allocator.allocate();
                    prototypes::player(&mut change, pc, Vector2::new(x, y));
                    prototypes::stone_floor(&mut change, allocator.allocate(), Vector2::new(x, y));
                }
                _ => panic!(),
            }
            x += 1;
        }
        y += 1;
    }

    let time = 1;
    spatial_hash.update(&entity_store, &change, time);
    entity_store.commit_change(&mut change);

    let mut knowledge = PlayerKnowledgeGrid::new(spatial_hash.width(), spatial_hash.height());

    let shadowcast = shadowcast::Shadowcast::new();
    shadowcast.observe(
        *entity_store.position.get(&pc).unwrap(),
        &spatial_hash,
        10,
        &entity_store,
        time,
        &mut knowledge
    );

    let sdl = sdl2::init().expect("SDL2 initialization failed");
    let video = sdl.video().expect("Failed to connect to video subsystem");
    let mut renderer_env = RendererEnv::new(WIDTH_PX, HEIGHT_PX, &video);
    sdl2::image::init(INIT_PNG).expect("Failed to connect to image subsystem");

    let mut renderer = GameRenderer::new(WIDTH,
                                         HEIGHT,
                                         &mut renderer_env,
                                         "resources/tiles.png",
                                         "resources/tiles.toml");

    renderer.update(&knowledge, time);
    renderer.draw();
    renderer.publish();

    let mut event_pump = sdl.event_pump().expect("Failed to initialize event pump");

    loop {
        match event_pump.wait_event() {
            Event::Quit { .. } => break,
            Event::KeyDown { .. } => break,
            _ => {}
        }
    }
}
