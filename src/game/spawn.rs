use std::time::Instant;
use rand::Rng;
use crate::game::state::GameState;
use crate::game::entities::Enemy;

pub fn maybe_spawn(state: &mut GameState, now: Instant) {
    let since = now.saturating_duration_since(state.last_spawn);
    if since < state.spawn_cooldown {
        return;
    }
    state.last_spawn = now;

    let elapsed_s = state.elapsed.as_secs_f32();
    let p = (0.20 + elapsed_s * 0.01).min(0.80);
    let rng = &mut state.rng;
    if rng.random_bool(p as f64) {
        let lane = rng.random_range(0..3u8);
        let x = state.lane_x(lane);
        let speed_jitter = rng.random_range(0.0..=2.0);
        let e = Enemy {
            x,
            y: 1.0,
            speed: state.base_speed + speed_jitter,
            w: 1,
            h: 1,
        };
        state.enemies.push(e);
    }

    let ms = state.spawn_cooldown.as_millis() as i64;
    let new_ms = (ms - 2).max(150) as u64;
    state.spawn_cooldown = std::time::Duration::from_millis(new_ms);
}