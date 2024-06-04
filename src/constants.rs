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
pub const STARTING_LIVES: u8 = 1;

// Map
pub const TOP_BOUND: f32 = 100.0;
pub const DOWNTIMER: f32 = 2.0;
pub const PLAYER_SPAWN_Y: f32 = 20.0;

/// Mushrooms
pub const MUSHROOM_SIZE: f32 = 11.0;
pub const MUSHROOM_MAX_AMOUNT: u8 = 100; // 100
pub const MUSHROOM_MIN_AMOUNT: u8 = 70; // 70
pub const SPAWN_MARGIN: f32 = 20.0;
pub const MUSHROOM_HEALTH: i8 = 3;
pub const MUSHROOM_CONVERSION_RATE: f64 = 1.0;
pub const MUSHROOM_POISON_COLOR: Color = Color::rgb(0.2, 0.2, 1.0);

/// Millipede
pub const MILLIPEDE_STARTING_LENGTH: usize = 9;
pub const MILLIPEDE_MAX_LENGTH: usize = 40;
pub const MILLIPEDE_SPEED: f32 = 300.0;
pub const MILLIPEDE_MAX_SPEED: f32 = 500.0;
pub const MILLIPEDE_SPEED_CHANGE_RATE: f32 = 1.025;
pub const SEGMENT_SIZE: f32 = 16.0;
pub const SEGMENT_SPACING: f32 = 15.0;
pub const DROP_AMOUNT: f32 = SEGMENT_SIZE / 2.0 + 1.0;
pub const PUSH_BACK_AMOUNT: f32 = 10.0;
pub const SEGMENT_SPAWN_TIMER_DURATION: f32 = 9.0;
pub const SEGMENT_DROP_RATE: f64 = 0.1;

// Explosions
pub const EXPLOSION_DURATION: f32 = 0.1;
pub const EXPLOSION_SIZE: f32 = 22.0;

// Beetle
pub const BEETLE_SPEED: f32 = 500.0;
pub const BEETLE_SPAWN_RATE: u8 = 5;

// Spider
pub const SPIDER_TIMER: f32 = 10.0;
pub const SPIDER_SPAWN_RATE: f64 = 0.5;
pub const SPIDER_AVERAGE_SPAWN_HEIGHT: f32 = 500.0;
pub const SPIDER_DIRECTION_CHANGE_RATE: f64 = 0.07;
pub const SPIDER_SPEED: f32 = 275.0;
pub const SPIDER_SIZE: f32 = 16.0;
pub const SPIDER_ATTACK_RATE: f64 = 0.1;
pub const SPIDER_LEAVE_RATE: f64 = 0.8;
pub const SPIDER_EAT_RATE: f64 = 0.05;

// Scorpion
pub const SCORPION_SPAWN_RATE: f64 = 0.0005;
pub const SCORPION_SPAWN_HEIGHT: f32 = 100.0;
pub const SCORPION_SPEED: f32 = 250.0;
pub const SCORPION_SIZE: f32 = 16.0;

// Scoring
pub const MUSHROOM_REWARD: u32 = 1;
pub const SEGMENT_REWARD: u32 = 10;
pub const HEAD_REWARD: u32 = 100;
pub const BEETLE_REWARD: u32 = 15;
pub const SPIDER_REWARD: u32 = 500;
pub const SCORPION_REWARD: u32 = 1000;

// UI
pub const TEXT_COLOR: Color = Color::rgb(102.0 / 255.0, 255.0 / 255.0, 143.0 / 255.0);
pub const TEXT_BACKGROUND: Color = Color::rgba(0.0 / 255.0, 51.0 / 255.0, 0.0 / 255.0, 1.0);
pub const TEXT_SIZE: f32 = 20.0;
pub const TOP_UI_HEIGHT: f32 = 24.0;
pub const BUTTON_HOVER_COLOR: Color = Color::rgba(129.0 / 255.0, 161.0 / 255.0, 137.0 / 255.0, 0.2);
pub const BUTTON_NORMAL_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

// Audio
pub const BACKGROUND_VOLUME: f64 = 0.7;
pub const MILLIPEDE_VOLUME: f64 = 0.6;
pub const SPIDER_VOLUME: f64 = 0.4;
pub const SCORPION_VOLUME: f64 = 0.4;
