use super::*;
use crate::audio::AudioHandles;
use bevy_kira_audio::{Audio, AudioControl};

#[derive(Component)]
pub struct PlayerProjectile;

impl PlayerProjectile {
    pub fn spawn(
        location_transform: &Transform,
        commands: &mut Commands,
        game_assets: &Res<GameAssets>,
    ) {
        let projectile_texture = &game_assets.projectile_texture;

        commands.spawn((
            PlayerProjectile,
            SpriteBundle {
                texture: projectile_texture.clone(),
                transform: *location_transform,
                ..default()
            },
        ));
    }
}

pub fn shoot_projectile(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    projectile_query: Query<Entity, With<PlayerProjectile>>,
    input: Res<ButtonInput<KeyCode>>,
    game_assets: Res<GameAssets>,
    audio: Res<Audio>,
    audio_handles: Res<AudioHandles>,
) {
    // Check if a projectile already exists
    if !projectile_query.is_empty() {
        return;
    }

    // Check if shoot key is pressed
    if !(input.pressed(SHOOT_KEY)) {
        return;
    }
    if let Ok(player_transform) = player_query.get_single() {
        PlayerProjectile::spawn(&player_transform, &mut commands, &game_assets);
        // Play shoot sound
        audio.play(audio_handles.shoot.clone()).with_volume(0.2);
    }
}

pub fn move_projectile(
    mut projectile_query: Query<&mut Transform, With<PlayerProjectile>>,
    time: Res<Time>,
) {
    for mut transform in projectile_query.iter_mut() {
        // Move upwards
        transform.translation.y +=
            PROJECTILE_SPEED * PROJECTILE_ACCELERATION * time.delta_seconds();
    }
}

pub fn despawn_projectile(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<PlayerProjectile>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Get window height
    let window = window_query.get_single().unwrap();
    let max_height = window.height() - PLAYER_SIZE;
    for (entity, transform) in projectile_query.iter() {
        // Check if hit ceiling
        if transform.translation.y >= max_height {
            // Despawn it
            commands.entity(entity).despawn();
        }
    }
}

pub fn projectile_hits_shroom(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<PlayerProjectile>>,
    mut mushroom_query: Query<(&mut Health, &Transform), With<Mushroom>>,
    mut score: ResMut<Score>,
) {
    if let Ok((projectile_entity, projectile_transform)) = projectile_query.get_single() {
        for (mut mushroom_health, mushroom_transform) in mushroom_query.iter_mut() {
            let projectile_radius = PROJECTILE_SIZE / 2.0;
            let mushroom_radius = MUSHROOM_SIZE / 2.0;

            let distance = projectile_transform
                .translation
                .distance(mushroom_transform.translation);
            if distance < projectile_radius + mushroom_radius {
                commands.entity(projectile_entity).despawn();
                if mushroom_health.0 - 1 == 0 {
                    score.0 += MUSHROOM_REWARD;
                }
                mushroom_health.0 -= 1;
                break;
            }
        }
    }
}

pub fn projectile_hits_segment(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<PlayerProjectile>>,
    segment_query: Query<(Entity, &Transform, &Segment)>,
    mut event_writer: EventWriter<DespawnSegment>,
    mut score: ResMut<Score>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut spawn_mushroom_ew: EventWriter<SpawnMushroomEvent>,
) {
    if let Ok((projectile_entity, projectile_transform)) = projectile_query.get_single() {
        for (segment_entity, segment_transform, segment) in segment_query.iter() {
            let projectile_radius = PROJECTILE_SIZE / 2.0;
            let segment_radius = SEGMENT_SIZE / 2.0;

            let distance = projectile_transform
                .translation
                .distance(segment_transform.translation);
            if distance < projectile_radius + segment_radius {
                // Pass in the direction if its a head
                if let Segment::Head {
                    direction,
                    head_state: _,
                } = segment
                {
                    event_writer.send(DespawnSegment {
                        entity: segment_entity,
                        direction: Some(*direction),
                    });
                } else {
                    event_writer.send(DespawnSegment {
                        entity: segment_entity,
                        direction: None,
                    });
                }

                // Spawn explosion
                explosion_events.send(ExplosionEvent(segment_transform.clone()));

                // Spawn mushroom
                spawn_mushroom_ew.send(SpawnMushroomEvent(
                    segment_transform.clone(),
                    Color::rgb(1.0, 1.0, 1.0),
                ));

                commands.entity(projectile_entity).despawn();
                commands.entity(segment_entity).despawn();

                // Add to score
                match segment {
                    Segment::Head {
                        direction: _,
                        head_state: _,
                    } => {
                        score.0 += HEAD_REWARD;
                    }
                    Segment::Body { parent: _ } => {
                        score.0 += SEGMENT_REWARD;
                    }
                }

                return;
            }
        }
    }
}

pub fn projectile_hits_beetle(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<PlayerProjectile>>,
    beetle_query: Query<(Entity, &Transform), With<Beetle>>,
    mut score: ResMut<Score>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut spawn_mushroom_ew: EventWriter<SpawnMushroomEvent>,
    mut score_event: EventWriter<FloatingScoreEvent>,
) {
    if let Ok((projectile_entity, projectile_transform)) = projectile_query.get_single() {
        for (beetle_entity, beetle_transform) in beetle_query.iter() {
            let projectile_radius = PROJECTILE_SIZE / 2.0;
            let segment_radius = SEGMENT_SIZE / 2.0;

            let distance = projectile_transform
                .translation
                .distance(beetle_transform.translation);
            if distance < projectile_radius + segment_radius {
                // Spawn explosion
                explosion_events.send(ExplosionEvent(beetle_transform.clone()));
                // Spawn mushroom
                spawn_mushroom_ew.send(SpawnMushroomEvent(
                    beetle_transform.clone(),
                    Color::rgb(1.0, 1.0, 1.0),
                ));
                // Send scoreUI event
                score_event.send(FloatingScoreEvent(beetle_transform.clone(), BEETLE_REWARD));

                commands.entity(projectile_entity).despawn();
                commands.entity(beetle_entity).despawn();

                // Add to score
                score.0 += BEETLE_REWARD;

                return;
            }
        }
    }
}

pub fn projectile_hits_spider(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<PlayerProjectile>>,
    spider_query: Query<(Entity, &Transform), With<Spider>>,
    mut score: ResMut<Score>,
    spider_timer: ResMut<SpiderTimer>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut score_event: EventWriter<FloatingScoreEvent>,
) {
    if let Ok((projectile_entity, projectile_transform)) = projectile_query.get_single() {
        for (spider_entity, spider_transform) in spider_query.iter() {
            let projectile_radius = PROJECTILE_SIZE / 2.0;
            let segment_radius = SEGMENT_SIZE / 2.0;

            let distance = projectile_transform
                .translation
                .distance(spider_transform.translation);
            if distance < projectile_radius + segment_radius {
                let calculated_score =
                    (spider_transform.translation.y * 0.8) as u32 + SPIDER_REWARD;

                // Spawn explosion
                explosion_events.send(ExplosionEvent(spider_transform.clone()));
                // Send scoreUI event
                score_event.send(FloatingScoreEvent(
                    spider_transform.clone(),
                    calculated_score,
                ));

                commands.entity(projectile_entity).despawn();

                Spider::despawn(spider_entity, &mut commands, spider_timer);

                // Add to score
                score.0 += calculated_score;

                return;
            }
        }
    }
}

pub fn projectile_hits_scorpion(
    mut commands: Commands,
    scorpion_query: Query<(Entity, &Transform), With<Scorpion>>,
    projectile_query: Query<(Entity, &Transform), With<PlayerProjectile>>,
    mut score: ResMut<Score>,
    mut explosion_events: EventWriter<ExplosionEvent>,
    mut score_event: EventWriter<FloatingScoreEvent>,
) {
    if let (
        Ok((projectile_entity, projectile_transform)),
        Ok((scorpion_entity, scorpion_transform)),
    ) = (projectile_query.get_single(), scorpion_query.get_single())
    {
        let projectile_radius = PROJECTILE_SIZE / 2.0;
        let scorpion_radius = SCORPION_SIZE / 2.0;

        let distance = projectile_transform
            .translation
            .distance(scorpion_transform.translation);

        if distance > projectile_radius + scorpion_radius {
            return;
        }

        // Spawn explosion
        explosion_events.send(ExplosionEvent(scorpion_transform.clone()));
        // Send scoreUI event
        score_event.send(FloatingScoreEvent(
            scorpion_transform.clone(),
            SCORPION_REWARD,
        ));

        //Despawn projectile
        commands.entity(projectile_entity).despawn();
        // Kill Scorpion
        Scorpion::kill(scorpion_entity, &mut commands, &mut score);
    }
}
