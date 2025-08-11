use std::time::{Duration, Instant};
use crate::game::state::GameState;
use crate::game::scene::GameScene;
use crate::game::spawn::maybe_spawn;

fn aabb_intersect(ax: u16, ay: u16, aw: u16, ah: u16, bx: u16, by: u16, bw: u16, bh: u16) -> bool {
    let ax2 = ax.saturating_add(aw.saturating_sub(1));
    let ay2 = ay.saturating_add(ah.saturating_sub(1));
    let bx2 = bx.saturating_add(bw.saturating_sub(1));
    let by2 = by.saturating_add(bh.saturating_sub(1));
    !(ax2 < bx || bx2 < ax || ay2 < by || by2 < ay)
}

pub fn update(state: &mut GameState, dt: Duration) {
    if state.scene != GameScene::Running {
        return;
    }

    state.elapsed += dt;
    let dt_s = dt.as_secs_f32();
    let speed_scale = 1.0 + (state.elapsed.as_secs_f32() * 0.02);

    // Move enemies downward
    for e in &mut state.enemies {
        e.y += e.speed * speed_scale * dt_s;
    }

    // Move projectiles upward
    for p in &mut state.projectiles {
        p.y -= p.speed * dt_s;
    }

    // Cleanup off-screen
    let h = state.height;
    state.enemies.retain(|e| (e.y as i32) >= 1 && (e.y as u16) < h.saturating_sub(1));
    state.projectiles.retain(|p| (p.y as i32) >= 1);

    // Projectile vs Enemy collisions (simple same-cell overlap)
    let mut to_remove_proj = vec![false; state.projectiles.len()];
    let mut to_remove_enemy = vec![false; state.enemies.len()];
    for (pi, p) in state.projectiles.iter().enumerate() {
        let px = p.x;
        let py = p.y.round().clamp(1.0, (h.saturating_sub(2)) as f32) as u16;
        for (ei, e) in state.enemies.iter().enumerate() {
            let ex = e.x;
            let ey = e.y.round().clamp(1.0, (h.saturating_sub(2)) as f32) as u16;
            if aabb_intersect(px, py, 1, 1, ex, ey, e.w, e.h) {
                to_remove_proj[pi] = true;
                to_remove_enemy[ei] = true;
                // score
                // Protect against potential overflow (u64 is large, but be safe)
                state.score = state.score.saturating_add(100);
                break;
            }
        }
    }
    // Remove flagged
    if to_remove_enemy.iter().any(|&b| b) {
        let mut i = 0;
        state.enemies.retain(|_| {
            let keep = !to_remove_enemy[i];
            i += 1;
            keep
        });
    }
    if to_remove_proj.iter().any(|&b| b) {
        let mut i = 0;
        state.projectiles.retain(|_| {
            let keep = !to_remove_proj[i];
            i += 1;
            keep
        });
    }

    // Player vs Enemy collisions
    let now = Instant::now();
    let invuln = state.invuln_until;
    let player_x = state.player.x;
    let player_y = state.player.y;
    if invuln.map_or(true, |t| now >= t) {
        let mut hit_index: Option<usize> = None;
        for (i, e) in state.enemies.iter().enumerate() {
            let ex = e.x;
            let ey = e.y.round().clamp(1.0, (h.saturating_sub(2)) as f32) as u16;
            if aabb_intersect(player_x, player_y, 1, 1, ex, ey, e.w, e.h) {
                hit_index = Some(i);
                break;
            }
        }
        if let Some(i) = hit_index {
            // remove enemy and apply damage
            state.enemies.swap_remove(i);
            state.lives = state.lives.saturating_sub(1);
            state.invuln_until = Some(now + Duration::from_millis(1000));
            if state.lives == 0 {
                state.scene = GameScene::GameOver;
            }
        }
    }

        // Spawning
    maybe_spawn(state, now);
}