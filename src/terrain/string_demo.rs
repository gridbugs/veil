use rand::Rng;
use cgmath::Vector2;
use entity_store::EntityStoreChange;
use entity_id_allocator::EntityIdAllocator;
use content::{prototypes, DoorState};
use terrain::TerrainMetadata;

pub fn generate<R: Rng>(change: &mut EntityStoreChange, allocator: &mut EntityIdAllocator, rng: &mut R) -> TerrainMetadata {

    let level_str = vec![
"##############################################",
"#=,,,,,,,,,,#,,,,,,,,,=#...........#,,,,,,,,,#",
"#,,,,,,,,,,,#,,,@,,,,,,#...........#,,,,z,,,,#",
"#,z,,,,,,,,,+,,,,,,,,,,+...........#,,,,,,,,,#",
"#,,,,,z,,,,,#,,,,,,,=,,#...........+,,,,,,,,,#",
"#,,,,,,,,,,,#,,,,,,,,,,#...........#,,,z,,,,,#",
"#,,,,,,,,,,,######+#####...........###########",
"#,,,,,,,,,,,#................................#",
"#,,,,,,,,,,,#..................z.............#",
"#####+#######................................#",
"#............................................#",
"#................##########+#########........#",
"#................#=,z,,#,,,,,,,,,,,,#........#",
"#................#,,,,,#,,,,,,,,,,,,#........#",
"#................###+###,,z,,,,,,,,,#........#",
"#................#,,,,,#,,,,,,,,,,,,+........#",
"#.........z......#,,,z,#,,,,,,,,,z,,#..z.....#",
"#................+,,,,,+,,,,,,,,,,,,#........#",
"#................#,,,,,#,,,,,,,,,,,,#........#",
"##############################################",
    ];

    let mut pc = 0;
    let mut y = 0;
    for row in level_str.iter() {
        let mut x = 0;
        for ch in row.chars() {
            match ch {
                '#' => {
                    prototypes::wall(change, allocator.allocate(), Vector2::new(x, y));
                    prototypes::stone_floor(change, allocator.allocate(), Vector2::new(x, y));
                }
                '.' => {
                    prototypes::stone_floor(change, allocator.allocate(), Vector2::new(x, y));
                }
                ',' => {
                    let id = allocator.allocate();
                    prototypes::stone_floor(change, id, Vector2::new(x, y));
                    change.inside.insert(id);
                }
                '@' => {
                    pc = allocator.allocate();
                    prototypes::player(change, pc, Vector2::new(x, y));
                    prototypes::stone_floor(change, allocator.allocate(), Vector2::new(x, y));
                }
                'z' => {
                    prototypes::undead(change, allocator.allocate(), Vector2::new(x, y));
                    let id = allocator.allocate();
                    prototypes::stone_floor(change, id, Vector2::new(x, y));
                    change.inside.insert(id);
                }
                '+' => {
                    prototypes::door(change, allocator.allocate(), Vector2::new(x, y), DoorState::Closed);
                    prototypes::stone_floor(change, allocator.allocate(), Vector2::new(x, y));
                }
                '=' => {
                    prototypes::page(change, allocator.allocate(), Vector2::new(x, y));
                    let id = allocator.allocate();
                    prototypes::stone_floor(change, id, Vector2::new(x, y));
                    change.inside.insert(id);
                }
                _ => panic!(),
            }

            if rng.next_f64() < 0.1 {
                prototypes::rain(change, allocator.allocate(), Vector2::new(x, y), rng);
            }

            x += 1;
        }
        y += 1;
    }

    TerrainMetadata {
        player_id: Some(pc),
    }
}
