use std::result;
use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2_frontend::turn::ActEnvPlayer;
use content::ActionType;
use meta_action::*;
use direction::Direction;
use cgmath::Vector2;
use straight_line::*;
use render_overlay::RenderOverlay;
use limits::LimitsRect;

#[derive(Debug)]
pub enum Error {
    RenderingFailed,
}
pub type Result<T> = result::Result<T, Error>;

pub fn player_turn<R: Rng>(env: &mut ActEnvPlayer<R>) -> Result<MetaAction> {

    env.render().map_err(|_| Error::RenderingFailed)?;

    loop {
        match env.input.wait_event_timeout(128) {
            Some(Event::Quit { .. }) => return Ok(MetaAction::External(External::Quit)),
            Some(Event::KeyDown { keycode: Some(keycode), .. }) => {
                let action = match keycode {
                    Keycode::Up => ActionType::Walk(env.entity_id, Direction::North),
                    Keycode::Down => ActionType::Walk(env.entity_id, Direction::South),
                    Keycode::Left => ActionType::Walk(env.entity_id, Direction::West),
                    Keycode::Right => ActionType::Walk(env.entity_id, Direction::East),
                    Keycode::F => {
                        let start = *env.entity_store.position.get(&env.entity_id).expect("Missing position");
                        aim(env, start);
                        ActionType::Null
                    }
                    _ => continue,
                };

                return Ok(MetaAction::Action(action));
            }
            None => {
                env.policy.on_frame(0, env.entity_store, env.spatial_hash, env.rng, env.change);
                *env.time += 1;
                env.spatial_hash.update(env.entity_store, env.change, *env.time);
                env.entity_store.commit_change(env.change);
                env.render().map_err(|_| Error::RenderingFailed)?;
            }
            _ => continue,
        }
    }
}

fn aim<R: Rng>(env: &mut ActEnvPlayer<R>, start: Vector2<i32>) -> Option<InfiniteAbsoluteLineTraverse> {
    let mut end = start;
    loop {
        let line = FiniteAbsoluteLineTraverse::new_between(start, end);
        let overlay = RenderOverlay {
            aim_line: line,
        };
        env.renderer.clear();
        env.renderer.draw();
        env.renderer.draw_overlay(overlay);
        env.renderer.publish();
        let change = match env.input.wait_event() {
            Event::KeyDown { keycode: Some(keycode), .. } => {
                match keycode {
                    Keycode::Up => Vector2::new(0, -1),
                    Keycode::Down => Vector2::new(0, 1),
                    Keycode::Left => Vector2::new(-1, 0),
                    Keycode::Right => Vector2::new(1, 0),
                    Keycode::Return => {
                        return Some(InfiniteAbsoluteLineTraverse::new_between(start, end));
                    }
                    _ => return None,
                }
            }
            _ => continue,
        };

        end = env.spatial_hash.saturate(end + change);
    }
}
