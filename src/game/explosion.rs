use crate::constants::EXPLOSION_DURATION;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

#[derive(Bundle)]
pub struct ExplosionBundle {
    pub duration: ExplosionTimer,
    pub sprite: SpriteBundle,
}

impl Default for ExplosionBundle {
    fn default() -> Self {
        println!("Spawning explosion!");
        Self {
            duration: ExplosionTimer(Timer::from_seconds(EXPLOSION_DURATION, TimerMode::Once)),
            sprite: SpriteBundle { ..default() },
        }
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
            println!("despawned");
        }
    }
}
