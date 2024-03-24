use bevy::prelude::*;

pub const MENU_KEY: KeyCode = KeyCode::Escape;

// Player Controls
pub const SHOOT_KEY: KeyCode = KeyCode::Space;
pub const LEFT: KeyCode = KeyCode::KeyA;
pub const RIGHT: KeyCode = KeyCode::KeyD;
pub const UP: KeyCode = KeyCode::KeyW;
pub const DOWN: KeyCode = KeyCode::KeyS;

pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_SIZE: f32 = 16.0;
pub const PROJECTILE_SPEED: f32 = 700.0;
pub const PROJECTILE_ACCELERATION: f32 = 2.0;

pub const TOP_BOUND: f32 = 100.0;
