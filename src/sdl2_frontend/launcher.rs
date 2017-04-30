use simple_file;
use sdl2_frontend::tile::*;
use entity_store::*;
use spatial_hash::*;
use content::prototypes;
use entity_id_allocator::*;
use cgmath::Vector2;

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
    let mut spatial_hash = SpatialHashTable::new(12, 6);

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
                    prototypes::player(&mut change, allocator.allocate(), Vector2::new(x, y));
                    prototypes::stone_floor(&mut change, allocator.allocate(), Vector2::new(x, y));
                }
                _ => panic!(),
            }
            x += 1;
        }
        y += 1;
    }

    spatial_hash.update(&entity_store, &change, 0);
    entity_store.commit(&mut change);

    println!("{:?}", tile_resolver);
}
