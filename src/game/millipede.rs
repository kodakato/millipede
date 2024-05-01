use super::*;
use bevy::utils::HashMap;

pub struct Body {
    parent: Option<Entity>,
}

pub struct Head {
    direction: Vec3, // A normalized vector
}

impl Default for Head {
    fn default() -> Self {
        Head {
            direction: Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

#[derive(Component)]
pub enum Segment {
    Head { direction: Vec3 },
    Body { parent: Option<Entity> },
}

#[derive(Event)]
pub struct DespawnSegment(pub Entity, pub f32);

#[derive(Resource)]
pub struct SegmentPositions(pub HashMap<Entity, Vec3>); // Pos, Vec

pub struct Millipede;

impl Millipede {
    pub fn spawn(
        length: usize,
        starting_transform: &Transform,
        commands: &mut Commands,
        game_assets: &Res<GameAssets>,
    ) {
        let millipede_texture = &game_assets.segment_texture;
        let mut parent_entity: Option<Entity> = Some(
            commands
                .spawn((
                    SpriteBundle {
                        texture: millipede_texture.clone(),
                        transform: *starting_transform,
                        ..default()
                    },
                    Name::from("MillipedeSegment"),
                    Segment::Head {
                        direction: Vec3::new(1.0, 0.0, 0.0),
                    },
                ))
                .id(),
        );

        for _ in 1..length {
            let entity: Entity = commands
                .spawn((
                    SpriteBundle {
                        texture: millipede_texture.clone(),
                        transform: *starting_transform,
                        ..default()
                    },
                    Name::from("MillipedeSegment"),
                    Segment::Body {
                        parent: parent_entity,
                    },
                ))
                .id();
            parent_entity = Some(entity);
        }
    }

    pub fn despawn(commands: &mut Commands, segment_query: &Query<Entity, With<Segment>>) {
        // Despawn each segment
        for segment_entity in segment_query.iter() {
            commands.entity(segment_entity).despawn();
        }
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

// Stores the position in a global hashmap in order for the
// children segments to know the position of their parents
pub fn segment_movement(
    segment_positions: Res<SegmentPositions>,
    mut query: Query<(&mut Segment, &mut Transform)>,
    game_vars: Res<GameVariables>,
    time: Res<Time>,
) {
    for (segment, mut transform) in query.iter_mut() {
        match *segment {
            Segment::Body { parent } => {
                if let Some(parent_entity) = parent {
                    if let Some(&parent_position) = segment_positions.0.get(&parent_entity) {
                        let distance_to_parent = transform.translation.distance(parent_position);
                        if distance_to_parent > SEGMENT_SPACING {
                            let direction_to_parent =
                                (parent_position - transform.translation).normalize();
                            transform.translation += direction_to_parent
                                * game_vars.millipede_speed
                                * time.delta_seconds();

                            // Ensure that the segment doesn't move too close to its parent
                            if transform.translation.distance(parent_position) < SEGMENT_SPACING {
                                transform.translation =
                                    parent_position - direction_to_parent * SEGMENT_SPACING;
                            }
                        }
                    }
                }
            }
            Segment::Head { direction } => {
                // Head segment logic
                transform.translation.x +=
                    direction.x * time.delta_seconds() * game_vars.millipede_speed;
                transform.translation.y +=
                    direction.y * time.delta_seconds() * game_vars.millipede_speed;
            }
        }
    }
}

pub fn change_direction(
    mut head_query: Query<(&mut Segment, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Get window for size
    let window = window_query.get_single().unwrap();
    let segment_radius = SEGMENT_SIZE / 2.0;
    for (mut segment, mut transform) in head_query.iter_mut() {
        if let Segment::Head { ref mut direction } = *segment {
            // Check if hit left boundary
            if transform.translation.x < 0.0 + segment_radius {
                *direction = Vec3::new(1.0, 0.0, 0.0);
                transform.translation.y -= DROP_AMOUNT;
            }

            // And right
            if transform.translation.x > window.width() - segment_radius {
                *direction = Vec3::new(-1.0, 0.0, 0.0);
                transform.translation.y -= DROP_AMOUNT;
            }
        }
    }
}

pub fn update_segment_parents(
    mut event_reader: EventReader<DespawnSegment>,
    mut segment_query: Query<&mut Segment>,
) {
    for despawn_event in event_reader.read() {
        for mut segment in segment_query.iter_mut() {
            if let Segment::Body { parent } = *segment {
                if parent == Some(despawn_event.0) {
                    *segment = Segment::Head {
                        direction: Vec3::new(despawn_event.1, 0.0, 0.0),
                    };
                }
            }
        }
    }
}

pub fn collide_with_shroom(
    mut segment_query: Query<(&mut Transform, &mut Segment), Without<Mushroom>>,
    shroom_query: Query<&Transform, With<Mushroom>>,
) {
    let shroom_radius = MUSHROOM_SIZE / 3.0;
    let segment_radius = SEGMENT_SIZE / 3.0;
    for (mut segment_transform, mut segment) in segment_query.iter_mut() {
        if let Segment::Head { ref mut direction } = *segment {
            for shroom_transform in shroom_query.iter() {
                let distance = shroom_transform
                    .translation
                    .distance(segment_transform.translation);
                if distance < shroom_radius + segment_radius {
                    // Move down
                    segment_transform.translation.y -= DROP_AMOUNT;

                    // Reverse direction
                    direction.x = -direction.x;

                    // Bounce backwards slightly
                    segment_transform.translation.x += direction.x * 12.0;
                }
            }
        }
    }
}

pub fn segment_hits_player(
    mut commands: Commands,
    player_q: Query<(Entity, &Transform), With<Player>>,
    segment_q: Query<&Transform, With<Segment>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    game_assets: Res<GameAssets>,
    mut down_timer: ResMut<DownTimer>,
) {
    let player_radius = PLAYER_SIZE / 2.0;
    let segment_radius = SEGMENT_SIZE / 2.0;
    if let Ok((player_entity, player_transform)) = player_q.get_single() {
        for segment_transform in segment_q.iter() {
            let distance = player_transform
                .translation
                .distance(segment_transform.translation);
            if distance < player_radius + segment_radius {
                Player::kill(
                    player_transform,
                    player_entity,
                    &mut next_player_state,
                    &game_assets,
                    &mut commands,
                    &mut down_timer,
                );
            }
        }
    }
}
