use std::time::{Duration, Instant};
use ratatui::prelude::Rect;
use rand::{rngs::StdRng, SeedableRng};
use crate::game::scene::GameScene;
use crate::game::entities::{Player, Enemy, Projectile};

pub struct GameState {
    pub width: u16,
    pub height: u16,
    pub scene: GameScene,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub score: u64,
    pub lives: u8,
    pub invuln_until: Option<Instant>,
    pub rng: StdRng,
    pub last_spawn: Instant,
    pub spawn_cooldown: Duration,
    pub base_speed: f32,
    pub elapsed: Duration,
}

impl GameState {
    pub fn new(area: Rect, seed: u64) -> Self {
        let width = area.width.max(10);
        let height = area.height.max(10);
        let mut s = Self {
            width,
            height,
            scene: GameScene::Running,
            player: Player {
                lane: 1,
                x: 0,
                y: height.saturating_sub(3).max(1),
                fire_cooldown_until: Instant::now(),
            },
            enemies: Vec::new(),
            projectiles: Vec::new(),
            score: 0,
            lives: 3,
            invuln_until: None,
            rng: StdRng::seed_from_u64(seed),
            last_spawn: Instant::now(),
            spawn_cooldown: Duration::from_millis(600),
            base_speed: 10.0,
            elapsed: Duration::ZERO,
        };
        s.player.x = s.lane_x(s.player.lane);
        s
    }

    pub fn lane_x(&self, lane: u8) -> u16 {
        let lane = lane.min(2);
        if self.width <= 2 {
            return 0;
        }
        let inner_left = 1u16;
        let inner_width = self.width - 2;
        let inner_w = inner_width as u32;
        let center = inner_left as u32 + (inner_w * (2 * lane as u32 + 1)) / (2 * 3);
        center.min((self.width - 1) as u32) as u16
    }

    pub fn clamp_player(&mut self) {
        self.player.y = self.player.y.clamp(1, self.height.saturating_sub(2));
        self.player.x = self.lane_x(self.player.lane);
    }

    pub fn resize(&mut self, area: Rect) {
        self.width = area.width.max(10);
        self.height = area.height.max(10);
        self.player.y = self.player.y.min(self.height.saturating_sub(2)).max(1);
        self.player.x = self.lane_x(self.player.lane);
        self.enemies.retain(|e| (e.y as i32) >= 1 && (e.y as u16) < self.height.saturating_sub(1));
        self.projectiles.retain(|p| (p.y as i32) >= 1 && (p.y as u16) < self.height.saturating_sub(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lane_centers_monotonic() {
        let s = GameState::new(Rect::new(0,0, 80, 24), 42);
        let x0 = s.lane_x(0);
        let x1 = s.lane_x(1);
        let x2 = s.lane_x(2);
        assert!(x0 < x1 && x1 < x2);
        assert!(x0 > 0 && x2 < s.width-1);
    }
}