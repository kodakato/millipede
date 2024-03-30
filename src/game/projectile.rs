use bevy::{prelude::*, window::PrimaryWindow};

use super::{
    explosion::ExplosionBundle,
    millipede::{DespawnSegment, Segment},
    player::Player,
    shroom::Mushroom,
};

use crate::constants::*;

#[derive(Component)]
pub struct PlayerProjectile;

pub fn shoot_projectile(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    projectile_query: Query<Entity, With<PlayerProjectile>>,
    input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
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
        let projectile_texture = asset_server.load("stinger.png");
        let projectile_transform = *player_transform;
        commands.spawn((
            Name::from("Player Projectile"),
            PlayerProjectile,
            SpriteBundle {
                texture: projectile_texture,
                transform: projectile_transform,
                ..default()
            },
        ));
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
    mushroom_query: Query<(Entity, &Transform), With<Mushroom>>,
) {
    if let Ok((projectile_entity, projectile_transform)) = projectile_query.get_single() {
        for (mushroom_entity, mushroom_transform) in mushroom_query.iter() {
            let projectile_radius = PROJECTILE_SIZE / 2.0;
            let mushroom_radius = MUSHROOM_SIZE / 2.0;

            let distance = projectile_transform
                .translation
                .distance(mushroom_transform.translation);
            if distance < projectile_radius + mushroom_radius {
                commands.entity(projectile_entity).despawn();
                commands.entity(mushroom_entity).despawn();
                return;
            }
        }
    }
}

pub fn projectile_hits_segment(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<PlayerProjectile>>,
    segment_query: Query<(Entity, &Transform), With<Segment>>,
    mut event_writer: EventWriter<DespawnSegment>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((projectile_entity, projectile_transform)) = projectile_query.get_single() {
        for (segment_entity, segment_transform) in segment_query.iter() {
            let projectile_radius = PROJECTILE_SIZE / 2.0;
            let segment_radius = SEGMENT_SIZE / 2.0;

            let distance = projectile_transform
                .translation
                .distance(segment_transform.translation);
            if distance < projectile_radius + segment_radius {
                event_writer.send(DespawnSegment(segment_entity));

                let explosion_texture = asset_server.load("explosion.png");
                // Spawn explosion
                commands.spawn(
                    ExplosionBundle::default()
                        .with_texture(explosion_texture)
                        .with_transform(segment_transform),
                );

                let shroom_texture = asset_server.load("shroom.png");
                // Spawn mushroom in place
                commands.spawn((
                    SpriteBundle {
                        texture: shroom_texture,
                        transform: segment_transform.clone(),
                        ..default()
                    },
                    Mushroom,
                    Name::from("Mushroom"),
                ));

                commands.entity(projectile_entity).despawn();
                return;
            }
        }
    }
}
