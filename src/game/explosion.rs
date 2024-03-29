use crate::constants::{EXPLOSION_DURATION, EXPLOSION_SIZE};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

#[derive(Bundle)]
pub struct ExplosionBundle {
    pub duration: ExplosionTimer,
    pub sprite_bundle: SpriteBundle,
}

impl Default for ExplosionBundle {
    fn default() -> Self {
        Self {
            duration: ExplosionTimer(Timer::from_seconds(EXPLOSION_DURATION, TimerMode::Once)),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(EXPLOSION_SIZE, EXPLOSION_SIZE)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

impl ExplosionBundle {
    pub fn with_texture(mut self, texture: Handle<Image>) -> Self {
        self.sprite_bundle.texture = texture;
        self
    }
    pub fn with_transform(mut self, transform: &Transform) -> Self {
        self.sprite_bundle.transform = transform.clone();
        self
    }
}

pub fn despawn_explosions(
    mut explosion_query: Query<(Entity, &mut ExplosionTimer), With<ExplosionTimer>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut explosion_timer) in explosion_query.iter_mut() {
        explosion_timer
            .0
            .tick(Duration::from_secs_f32(time.delta_seconds()));
        if explosion_timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
