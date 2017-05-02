use cgmath::Vector2;
use simple_file;
use sdl2_frontend::tile::*;
use sdl2_frontend::tile_buffer::*;
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

    let tile_desc_str = simple_file::read_string("resources/tiles.toml")
        .expect("Failed to open tile description");
    let tile_resolver = TileResolver::from_str(&tile_desc_str);

    let mut entity_store = EntityStore::new();
    let mut change = EntityStoreChange::new();
    let mut allocator = EntityIdAllocator::new();
    let mut spatial_hash = SpatialHashTable::new(WIDTH, HEIGHT);

    let mut pc = EntityId::new(0);
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

    let mut buffer = TileBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    buffer.update(&knowledge, &tile_resolver, time, *entity_store.position.get(&pc).unwrap());

    println!("{:?}", pc);
    println!("{:?}", tile_resolver);
    println!("{:?}", knowledge);
    println!("{:?}", buffer);
}
