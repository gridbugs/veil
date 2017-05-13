use sdl2;
use sdl2::image::INIT_PNG;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use cgmath::Vector2;
use rand::{Rng, StdRng};
use sdl2_frontend::renderer::*;
use sdl2_frontend::renderer_env::*;
use entity_store::*;
use spatial_hash::*;
use content::prototypes;
use content::ActionType;
use content::DoorState;
use entity_id_allocator::*;
use knowledge::*;
use observation::*;
use policy::GamePolicy;
use direction::Direction;

const WIDTH: usize = 12;
const HEIGHT: usize = 6;

const WIDTH_PX: u32 = 800;
const HEIGHT_PX: u32 = 600;

pub fn launch() {

    let level_str = vec![
        "############",
        "#..#,,,,#..#",
        "#..-,,,,#..#",
        "#.@#+####..#",
        "#..........#",
        "############",
    ];

    let mut entity_store = EntityStore::new();
    let mut change = EntityStoreChange::new();
    let mut allocator = EntityIdAllocator::new();
    let mut spatial_hash = SpatialHashTable::new(WIDTH, HEIGHT);

    let mut rng = StdRng::new().unwrap();

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
                ',' => {
                    let id = allocator.allocate();
                    prototypes::stone_floor(&mut change, id, Vector2::new(x, y));
                    change.inside.insert(id);
                }
                '@' => {
                    pc = allocator.allocate();
                    prototypes::player(&mut change, pc, Vector2::new(x, y));
                    prototypes::stone_floor(&mut change, allocator.allocate(), Vector2::new(x, y));
                }
                '+' => {
                    prototypes::door(&mut change, allocator.allocate(), Vector2::new(x, y), DoorState::Closed);
                    prototypes::stone_floor(&mut change, allocator.allocate(), Vector2::new(x, y));
                }
                '-' => {
                    prototypes::door(&mut change, allocator.allocate(), Vector2::new(x, y), DoorState::Open);
                    prototypes::stone_floor(&mut change, allocator.allocate(), Vector2::new(x, y));
                }
                _ => panic!(),
            }

            if rng.next_f64() < 0.1 {
                prototypes::rain(&mut change, allocator.allocate(), Vector2::new(x, y));
            }

            x += 1;
        }
        y += 1;
    }

    let mut time = 1;
    spatial_hash.update(&entity_store, &change, time);
    entity_store.commit_change(&mut change);

    let mut knowledge = PlayerKnowledgeGrid::new(spatial_hash.width(), spatial_hash.height());

    let policy = GamePolicy;

    let shadowcast = shadowcast::Shadowcast::new();
    let sdl = sdl2::init().expect("SDL2 initialization failed");
    let video = sdl.video().expect("Failed to connect to video subsystem");
    let mut renderer_env = RendererEnv::new(WIDTH_PX, HEIGHT_PX, &video);
    sdl2::image::init(INIT_PNG).expect("Failed to connect to image subsystem");

    let mut renderer = GameRenderer::new(WIDTH,
                                         HEIGHT,
                                         &mut renderer_env,
                                         "resources/tiles.png",
                                         "resources/tiles.toml");
    let mut event_pump = sdl.event_pump().expect("Failed to initialize event pump");

    let mut reactions = Vec::new();

    'outer: loop {

        shadowcast.observe(
            *entity_store.position.get(&pc).unwrap(),
            &spatial_hash,
            10,
            &entity_store,
            time,
            &mut knowledge
        );

        renderer.update(&knowledge, time);
        renderer.draw();
        renderer.publish();

        'inner: loop {
            match event_pump.wait_event_timeout(128) {
                Some(Event::Quit { .. }) => break 'outer,
                Some(Event::KeyDown { keycode: Some(keycode), .. }) => {
                    let action = match keycode {
                        Keycode::Up => ActionType::Walk(pc, Direction::North),
                        Keycode::Down => ActionType::Walk(pc, Direction::South),
                        Keycode::Left => ActionType::Walk(pc, Direction::West),
                        Keycode::Right => ActionType::Walk(pc, Direction::East),
                        _ => continue 'inner,
                    };

                    reactions.push(action);

                    while let Some(action) = reactions.pop() {
                        action.populate(&mut change, &entity_store);

                        if policy.on_action(&change, &entity_store, &spatial_hash, &mut reactions) {
                            time += 1;
                            spatial_hash.update(&entity_store, &change, time);
                            entity_store.commit_change(&mut change);
                        } else {
                            change.clear();
                        }
                    }

                    break 'inner;
                }
                Some(_) => {}
                None => {
                    policy.on_tick(&entity_store, &spatial_hash, &mut rng, &mut change);
                    time += 1;
                    spatial_hash.update(&entity_store, &change, time);
                    entity_store.commit_change(&mut change);
                    break 'inner;
                }
            }
        }
    }
}
