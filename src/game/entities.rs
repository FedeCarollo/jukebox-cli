use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Player {
    pub lane: u8,             // 0..=2
    pub x: u16,               // column (relative to game area)
    pub y: u16,               // row (relative to game area)
    pub fire_cooldown_until: Instant,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub x: u16,
    pub y: f32,
    pub speed: f32, // rows per second
    pub w: u16,
    pub h: u16,
}

#[derive(Debug, Clone)]
pub struct Projectile {
    pub x: u16,
    pub y: f32,
    pub speed: f32, // rows per second upward
}