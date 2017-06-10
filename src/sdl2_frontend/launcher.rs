use std::collections::HashMap;
use sdl2;
use sdl2::image::INIT_PNG;
use cgmath::Vector2;
use rand::{Rng, StdRng};
use sdl2_frontend::renderer::*;
use sdl2_frontend::renderer_env::*;
use sdl2_frontend::input::*;
use turn::*;
use entity_store::*;
use spatial_hash::*;
use content::prototypes;
use content::DoorState;
use content::VeilStepInfo;
use entity_id_allocator::*;
use knowledge::*;
use observation::*;
use policy::GamePolicy;
use behaviour::*;
use schedule::Schedule;
use frame::AnimationMode;
use veil_state::VeilState;
use meta_action::*;

const WIDTH_PX: u32 = 1200;
const HEIGHT_PX: u32 = 600;

pub fn launch() {

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
                'z' => {
                    prototypes::undead(&mut change, allocator.allocate(), Vector2::new(x, y));
                    let id = allocator.allocate();
                    prototypes::stone_floor(&mut change, id, Vector2::new(x, y));
                    change.inside.insert(id);
                }
                '+' => {
                    prototypes::door(&mut change, allocator.allocate(), Vector2::new(x, y), DoorState::Closed);
                    prototypes::stone_floor(&mut change, allocator.allocate(), Vector2::new(x, y));
                }
                '=' => {
                    prototypes::page(&mut change, allocator.allocate(), Vector2::new(x, y));
                    let id = allocator.allocate();
                    prototypes::stone_floor(&mut change, id, Vector2::new(x, y));
                    change.inside.insert(id);
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

    let mut turn_schedule = Schedule::new();
    let mut knowledge = HashMap::new();
    let mut behaviour = HashMap::new();
    let veil_step_info = VeilStepInfo {
        x: 0.02,
        y: 0.01,
        z: 0.02,
        min: -0.02,
        max: 0.02,
    };
    let mut veil_state = VeilState::new(spatial_hash.width(), spatial_hash.height(), &mut rng, &veil_step_info);

    let mut action_schedule = Schedule::new();

    for (id, period) in entity_store.turn_period.iter() {
        turn_schedule.insert(*id, *period);
        if *id != pc {
            behaviour.insert(*id, BehaviourState::new());
            knowledge.insert(*id, PlayerKnowledgeGrid::new(spatial_hash.width(), spatial_hash.height()));
        }
    }

    entity_store.veil_step_info.insert(pc, veil_step_info);

    let mut player_knowledge = PlayerKnowledgeGrid::new(spatial_hash.width(), spatial_hash.height());

    let mut policy = GamePolicy::new();

    let mut shadowcast = shadowcast::ShadowcastEnv::new();
    let sdl = sdl2::init().expect("SDL2 initialization failed");
    let video = sdl.video().expect("Failed to connect to video subsystem");
    let mut renderer_env = RendererEnv::new(WIDTH_PX, HEIGHT_PX, &video);
    sdl2::image::init(INIT_PNG).expect("Failed to connect to image subsystem");

    let mut renderer = SdlGameRenderer::new(spatial_hash.width(),
                                         spatial_hash.height(),
                                         &mut renderer_env,
                                         "resources/tiles.png",
                                         "resources/tiles.toml");
    let event_pump = sdl.event_pump().expect("Failed to initialize event pump");
    let mut input = SdlGameInput::new(event_pump, 60, AnimationMode::RealTime);
    let mut reactions = Vec::new();
    let mut action_schedule_entries = Vec::new();

    let mut behaviour_env = BehaviourEnv::new(spatial_hash.width(), spatial_hash.height());

    while let Some(entry) = turn_schedule.next() {

        let entity_id = entry.value;

        if let Some(veil_step_info) = entity_store.veil_step_info.get(&entity_id) {
            veil_state.step(&mut rng, veil_step_info);
        }
        if entity_store.player.contains(&entity_id) {
            policy.veil_update(&mut change, &entity_store, &spatial_hash, &veil_state);

            time += 1;
            spatial_hash.update(&entity_store, &change, time);
            entity_store.commit_change(&mut change);
        }

        let resolution = TurnEnv {
            renderer: &mut renderer,
            input: &mut input,
            action_schedule_entries: &mut action_schedule_entries,
            reactions: &mut reactions,
            change: &mut change,
            entity_store: &mut entity_store,
            id_allocator: &mut allocator,
            spatial_hash: &mut spatial_hash,
            behaviour_env: &mut behaviour_env,
            player_id: pc,
            entity_id: entity_id,
            player_knowledge: &mut player_knowledge,
            knowledge: &mut knowledge,
            behaviour: &mut behaviour,
            shadowcast: &mut shadowcast,
            time: &mut time,
            policy: &mut policy,
            rng: &mut rng,
            schedule: &mut action_schedule,
        }.take_turn().unwrap();

        match resolution {
            TurnResolution::Reschedule => {
                if let Some(period) = entity_store.turn_period.get(&entity_id) {
                    turn_schedule.insert(entity_id, *period);
                }
            }
            TurnResolution::External(_) => {
                return;
            }
            TurnResolution::NoEntity => (),
            TurnResolution::Debug(debug_action) => {
                if let Some(info) = entity_store.veil_step_info.get_mut(&pc) {
                    match debug_action {
                        DebugAction::ChangeVeilMin(min) => {
                            info.min += min;
                        }
                        DebugAction::ChangeVeilMax(max) => {
                            info.max += max;
                        }
                        DebugAction::ChangeVeilStep(v) => {
                            info.x += v.x;
                            info.y += v.y;
                            info.z += v.z;
                        }
                        _ => (),
                    }
                    println!("{:?}", info);
                }
                match debug_action {
                    DebugAction::TogglePlayerOmniscient => {
                        if entity_store.omniscient.contains(&pc) {
                            entity_store.omniscient.remove(&pc);
                        } else {
                            entity_store.omniscient.insert(pc);
                        }
                    }
                    _ => (),
                }
                turn_schedule.insert(entity_id, 0);
            }
        }
    }

    panic!("schedule is empty");
}
