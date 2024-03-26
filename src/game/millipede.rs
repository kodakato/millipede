use bevy::{prelude::*, window::PrimaryWindow};
use bevy::utils::HashMap;


use crate::constants::*;

#[derive(Component)]
pub struct Segment {
    parent: Option<Entity>,
}

#[derive(Resource)]
pub struct SegmentPositions(pub HashMap<Entity, Vec3>); // Pos, Vel

pub fn spawn_millipede(mut commands: Commands, asset_server: Res<AssetServer>) {
    let millipede_texture = asset_server.load("millipede.png");
    let mut parent_entity: Option<Entity> = Some(
        commands
            .spawn((
                SpriteBundle {
                    texture: millipede_texture.clone(),
                    transform: Transform::from_xyz(100.0, 200.0, 0.0),
                    ..default()
                },
                Name::from("MillipedeSegmentHead"),
                Segment { parent: None },
            ))
            .id(),
    );

    for _ in 1..NUM_OF_SEGMENTS {
        let entity: Entity = commands
            .spawn((
                SpriteBundle {
                    texture: millipede_texture.clone(),
                    transform: Transform::from_xyz(100.0, 200.0, 0.0),
                    ..default()
                },
                Name::from("MillipedeSegment"),
                Segment {
                    parent: parent_entity,
                },
            ))
            .id();
        parent_entity = Some(entity);
    }
}

pub fn update_positions(
    mut segment_positions: ResMut<SegmentPositions>,
    query: Query<(Entity, &Transform)>,
) {
    for (entity, transform) in query.iter() {
        segment_positions.0.insert(entity, transform.translation);
    }
}

pub fn segment_movement(
    segment_positions: Res<SegmentPositions>,
    mut query: Query<(&Segment, &mut Transform)>,
    time: Res<Time>,
) {
    for (segment, mut transform) in query.iter_mut() {
        if let Some(parent_entity) = segment.parent {
            if let Some(&parent_position) = segment_positions.0.get(&parent_entity) {
                let distance_to_parent = transform.translation.distance(parent_position);

                // Only move if the distance to the parent is greater than the threshold
                if distance_to_parent > SEGMENT_SPACING {
                    // Calculate a normalized direction vector towards the parent
                    let direction_to_parent = (parent_position - transform.translation).normalize();
                    // Move towards the parent
                    transform.translation += direction_to_parent * MILLIPEDE_SPEED * time.delta_seconds();

                    // Optional: limit the movement to not overshoot the parent
                    if transform.translation.distance(parent_position) < SEGMENT_SPACING {
                        transform.translation = parent_position - direction_to_parent * SEGMENT_SPACING;
                    }
                }
            }
        } else {
            // Head segment logic
            transform.translation.y += MILLIPEDE_SPEED * time.delta_seconds();
        }
    }
}
