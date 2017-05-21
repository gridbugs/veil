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
use behaviour::*;

const WIDTH_PX: u32 = 1200;
const HEIGHT_PX: u32 = 600;

enum State {
    ValidPath,
    NoPath,
}

pub fn launch() {

    let level_str = vec![
"##############################################",
"#,,,,,,,,,,,#,,,,,,,,,,#...........#,,,,,,,,,#",
"#,,,,,,,,,,,#,,,,,,,,,,#...........#,,,,,,,,,#",
"#,,,,,,,,,,,+,,,,,,,,,,+...........#,,,,,,,,,#",
"#,,,,,,,,,,,#,,,,,,,,,,#...........+,,,,,,,,,#",
"#,,,,,,,,,,,#,,,,,,,,,,#...........#,,,,,,,,,#",
"#,,,,,,,,,,,######+#####...........###########",
"#,,,,,,,,,,,#................................#",
"#,,,,,,,,,,,#................................#",
"#####+#######..........@.....................#",
"#............................................#",
"#................##########+#########........#",
"#................#,,,,,#,,,,,,,,,,,,#........#",
"#................#,,,,,#,,,,,,,,,,,,#........#",
"#................#######,,,,,,,,,,,,#........#",
"#................#,,,,,#,,,,,,,,,,,,+........#",
"#................#,,,,,#,,,,,,,,,,,,#........#",
"#................#,,,,,#,,,,,,,,,,,,#........#",
"#................#,,,,,#,,,,,,,,,,,,#........#",
"##############################################",
    ];

    let mut entity_store = EntityStore::new();
    let mut change = EntityStoreChange::new();
    let mut allocator = EntityIdAllocator::new();
    let mut spatial_hash = SpatialHashTable::new(level_str[0].len(), level_str.len());

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
                _ => panic!(),
            }

            if rng.next_f64() < 0.1 {
                prototypes::rain(&mut change, allocator.allocate(), Vector2::new(x, y), &mut rng);
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

    let mut renderer = GameRenderer::new(spatial_hash.width(),
                                         spatial_hash.height(),
                                         &mut renderer_env,
                                         "resources/tiles.png",
                                         "resources/tiles.toml");
    let mut event_pump = sdl.event_pump().expect("Failed to initialize event pump");

    let mut reactions = Vec::new();

    let mut behaviour_state = BehaviourState::new();
    let mut behaviour_env = BehaviourEnv::new(spatial_hash.width(), spatial_hash.height());

    'outer: loop {

        let position = *entity_store.position.get(&pc).unwrap();
        let metadata = shadowcast.observe(
            position,
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

                    /*
                    let should_search = match state {
                        State::ValidPath => {
                            if knowledge.is_visible(path.destination(), time) {
                                true
                            } else {
                                let mut should_search = false;
                                for step in path.iter_from(path_idx) {
                                    if let Some(cell) = knowledge.get(step.to_coord()) {
                                        if cell.solid {
                                            should_search = true;
                                            break;
                                        }
                                        if !cell.is_visible(time) {
                                            break;
                                        }
                                    }
                                }
                                should_search
                            }
                        }
                        State::NoPath => true,
                    };

                    if should_search {
                        bfs(&mut search_env, &knowledge, position, DirectionsCardinal, &mut path).expect("failed to search");
                        state = State::ValidPath;
                        path_idx = 0;
                    }

                    let step = path.get(path_idx).unwrap();
*/
                    let action = match keycode {
                        Keycode::Up => ActionType::Walk(pc, Direction::North),
                        Keycode::Down => ActionType::Walk(pc, Direction::South),
                        Keycode::Left => ActionType::Walk(pc, Direction::West),
                        Keycode::Right => ActionType::Walk(pc, Direction::East),
                        Keycode::Space => {
                            /*
                            path_idx += 1;
                            ActionType::Walk(pc, step.direction())
                            */
                            patrol::patrol(pc, &entity_store, &knowledge, metadata, time, &mut behaviour_env, &mut behaviour_state)
                        }
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
