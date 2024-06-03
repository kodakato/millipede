use super::*;
use crate::audio::AudioHandles;
use bevy_kira_audio::{Audio, AudioControl};
use std::time::Duration;

#[derive(Component)]
pub struct Explosion(pub Timer);

#[derive(Event)]
pub struct ExplosionEvent(pub Transform);


pub fn spawn_explosion(
    mut commands: Commands,
    mut explosion_events: EventReader<ExplosionEvent>,
    game_assets: Res<GameAssets>,
    audio: Res<Audio>,
    audio_handles: Res<AudioHandles>,
) {
    for event in explosion_events.read() {
        let explosion_texture = &game_assets.explosion_texture;

        commands.spawn((
            Explosion(Timer::new(
                Duration::from_secs_f32(EXPLOSION_DURATION),
                TimerMode::Once,
            )),
            SpriteBundle {
                texture: explosion_texture.clone(),
                transform: event.0,
                ..default()
            },
            Name::from("Explosion"),
        ));

        // Play sound
        audio.play(audio_handles.explosion.clone()).with_volume(0.4);
    }
}

pub fn despawn_explosions(
    mut explosion_query: Query<(Entity, &mut Explosion)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut explosion_timer) in explosion_query.iter_mut() {
        explosion_timer.0.tick(time.delta());
        if explosion_timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
