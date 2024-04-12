use bevy::prelude::*;

/// App Controls
pub const MENU_KEY: KeyCode = KeyCode::Escape;
pub const QUIT_KEY: KeyCode = KeyCode::KeyQ;

/// Player Controls
pub const SHOOT_KEY: KeyCode = KeyCode::Space;
pub const LEFT: KeyCode = KeyCode::KeyA;
pub const RIGHT: KeyCode = KeyCode::KeyD;
pub const UP: KeyCode = KeyCode::KeyW;
pub const DOWN: KeyCode = KeyCode::KeyS;

// Attributes
pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_SIZE: f32 = 16.0;

/// Projectile
pub const PROJECTILE_SPEED: f32 = 1300.0;
pub const PROJECTILE_ACCELERATION: f32 = 1.0;
pub const PROJECTILE_SIZE: f32 = 10.0;

// Map
pub const TOP_BOUND: f32 = 100.0;

/// Mushrooms
pub const MUSHROOM_SIZE: f32 = 11.0;
pub const MUSHROOM_MAX_AMOUNT: u8 = 100;
pub const MUSHROOM_MIN_AMOUNT: u8 = 70;
pub const SPAWN_MARGIN: f32 = 20.0;
pub const MUSHROOM_HEALTH: i8 = 3;

/// Millipede
pub const NUM_OF_SEGMENTS: u8 = 10;
pub const MILLIPEDE_SPEED: f32 = 250.0;
pub const SEGMENT_SIZE: f32 = 16.0;
pub const SEGMENT_SPACING: f32 = 13.0;
pub const DROP_AMOUNT: f32 = 10.0;

// Explosions
pub const EXPLOSION_DURATION: f32 = 0.1;
pub const EXPLOSION_SIZE: f32 = 22.0;

// Beetle
pub const BEETLE_SPEED: f32 = 500.0;
pub const BEETLE_SPAWN_RATE: u8 = 5;

// Scoring
pub const MUSHROOM_REWARD: u32 = 1;
pub const SEGMENT_REWARD: u32 = 10;
pub const HEAD_REWARD: u32 = 100;
pub const BEETLE_REWARD: u32 = 15;

// UI
pub const TEXT_COLOR: Color = Color::rgb(110.0/ 255.0, 255.0 / 255.0, 248.0 / 255.0);
pub const TEXT_TRANSPARENCY: f32 = 0.7;
