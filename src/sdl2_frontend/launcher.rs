use sdl2;
use sdl2::image::INIT_PNG;
use cgmath::Vector2;
use sdl2_frontend::renderer::*;
use entity_store::*;
use spatial_hash::*;
use content::prototypes;
use entity_id_allocator::*;
use knowledge::*;
use observation::*;

const WIDTH: usize = 12;
const HEIGHT: usize = 6;

const SCREEN_WIDTH: usize = 8;
const SCREEN_HEIGHT: usize = 8;

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

    square::observe(
        *entity_store.position.get(&pc).unwrap(),
        4,
        &spatial_hash,
        &entity_store,
        time,
        &mut knowledge);

    let sdl = sdl2::init().expect("SDL2 initialization failed");
    let video = sdl.video().expect("Failed to connect to video subsystem");
    sdl2::image::init(INIT_PNG).expect("Failed to connect to image subsystem");

    GameRenderer::new(SCREEN_WIDTH, SCREEN_HEIGHT, &video, "resources/tiles.png", "resources/tiles.toml");
}
