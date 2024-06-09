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
            SpriteSheetBundle {
                texture: explosion_texture.clone(),
                transform: Transform::from_xyz(event.0.translation.x, event.0.translation.y, 0.5).with_scale(Vec3::new(EXPLOSION_SIZE, EXPLOSION_SIZE, 0.0)),
                atlas: TextureAtlas {
                    layout: game_assets.explosion_layout.clone(),
                    index: EXPLOSION_ANIMATION_INDICES.first,
                },
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

pub fn animate_explosion(mut explosion_query: Query<(&mut TextureAtlas, &Explosion)>) {
    for (mut atlas, explosion) in explosion_query.iter_mut() {
        let elapsed = explosion.0.elapsed().as_secs_f32();
        let frame = ((elapsed / EXPLOSION_DURATION) * 3 as f32) as usize;
        let reversed_frame = 2 - frame; // Reverse the frame index
        atlas.index = reversed_frame;
    }
}
