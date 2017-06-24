use std::result;

use entity_store::EntityId;
use game_env::GameEnv;
use level_env::LevelEnv;
use terrain;
use content::VeilStepInfo;
use knowledge::PlayerKnowledgeGrid;
use behaviour::BehaviourState;
use turn::{self, TurnEnv, TurnResolution};
use renderer::GameRenderer;
use input::GameInput;
use meta_action::DebugAction;

#[derive(Debug)]
pub enum Error {
    TurnError(turn::Error),
}
pub type Result<T> = result::Result<T, Error>;

enum GameLoopExit {
    Quit,
}

pub fn launch<Ren: GameRenderer, Inp: GameInput>(renderer: &mut Ren, input: &mut Inp) {

    let mut game = GameEnv::new();

    let width = 80;
    let height = 30;

    let veil_step_info = VeilStepInfo {
        x: 0.02,
        y: 0.01,
        z: 0.02,
        min: -0.02,
        max: 0.02,
    };

    let mut level = LevelEnv::new(width, height, &mut game.rng, &veil_step_info);

    let md = terrain::string_demo::generate(&mut game.change, &mut game.id_allocator, &mut game.rng);
    let player_id = md.player_id.expect("missing player");

    level.commit(&mut game.change, game.time);

    init_player(player_id, &mut level, veil_step_info);
    game_loop(player_id, &mut game, &mut level, renderer, input).expect("Error in game loop");
}

fn init_player(player_id: EntityId, level: &mut LevelEnv, veil_step_info: VeilStepInfo) {
    for (id, period) in level.entity_store.turn_period.iter() {
        level.turn_schedule.insert(*id, *period);
        if *id != player_id {
            level.behaviour.insert(*id, BehaviourState::new());
            level.knowledge.insert(*id, PlayerKnowledgeGrid::new(level.spatial_hash.width(), level.spatial_hash.height()));
        }
    }

    level.entity_store.veil_step_info.insert(player_id, veil_step_info);
}

fn take_turn<Ren: GameRenderer, Inp: GameInput>(player_id: EntityId, entity_id: EntityId,
                                                game: &mut GameEnv, level: &mut LevelEnv,
                                                renderer: &mut Ren, input: &mut Inp)
    -> turn::Result<TurnResolution> {

    TurnEnv {
        renderer: renderer,
        input: input,
        action_schedule_entries: &mut game.action_schedule_entries,
        reactions: &mut game.reactions,
        change: &mut game.change,
        entity_store: &mut level.entity_store,
        id_allocator: &mut game.id_allocator,
        spatial_hash: &mut level.spatial_hash,
        behaviour_env: &mut level.behaviour_env,
        player_id: player_id,
        entity_id: entity_id,
        player_knowledge: &mut level.player_knowledge,
        knowledge: &mut level.knowledge,
        behaviour: &mut level.behaviour,
        shadowcast: &mut game.shadowcast,
        time: &mut game.time,
        policy: &mut game.policy,
        rng: &mut game.rng,
        schedule: &mut game.action_schedule,
    }.take_turn()
}

fn pre_turn(entity_id: EntityId, game: &mut GameEnv, level: &mut LevelEnv) {

    if let Some(veil_step_info) = level.entity_store.veil_step_info.get(&entity_id) {
        level.veil_state.step(&mut game.rng, veil_step_info);
    }

    if level.entity_store.player.contains(&entity_id) {
        game.policy.veil_update(&mut game.change,
                                &level.entity_store,
                                &level.spatial_hash,
                                &level.veil_state);

        game.time += 1;
        level.spatial_hash.update(&level.entity_store, &game.change, game.time);
        level.entity_store.commit_change(&mut game.change);
    }
}

fn handle_turn_resolution<Ren: GameRenderer>(resolution: TurnResolution,
                                             player_id: EntityId, entity_id: EntityId,
                                             level: &mut LevelEnv,
                                             renderer: &mut Ren) -> Option<GameLoopExit>  {
    match resolution {
        TurnResolution::Reschedule => {
            if let Some(period) = level.entity_store.turn_period.get(&entity_id) {
                level.turn_schedule.insert(entity_id, *period);
            }
        }
        TurnResolution::External(_) => {
            return Some(GameLoopExit::Quit);
        }
        TurnResolution::NoEntity => (),
        TurnResolution::Debug(debug_action) => {
            if let Some(info) = level.entity_store.veil_step_info.get_mut(&player_id) {
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
                    if level.entity_store.omniscient.contains(&player_id) {
                        level.entity_store.omniscient.remove(&player_id);
                    } else {
                        level.entity_store.omniscient.insert(player_id);
                    }
                }
                DebugAction::ToggleDiminishingLighting => {
                    let mut config = renderer.config();
                    config.diminishing_lighting = !config.diminishing_lighting;
                    renderer.set_config(config);
                }
                _ => (),
            }
            level.turn_schedule.insert(entity_id, 0);
        }
    }

    None
}

fn game_loop<Ren: GameRenderer, Inp: GameInput>(player_id: EntityId,
                                                game: &mut GameEnv, level: &mut LevelEnv,
                                                renderer: &mut Ren, input: &mut Inp) -> Result<GameLoopExit> {
    while let Some(entry) = level.turn_schedule.next() {
        let entity_id = entry.value;

        pre_turn(entity_id, game, level);

        let resolution = take_turn(player_id, entity_id,
                                   game, level,
                                   renderer, input).map_err(Error::TurnError)?;


        if let Some(exit) = handle_turn_resolution(resolution, player_id, entity_id, level, renderer) {
            return Ok(exit);
        }

    }

    Ok(GameLoopExit::Quit)
}
