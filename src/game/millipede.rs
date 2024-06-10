use super::*;
use bevy::utils::HashMap;
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
pub enum HeadState {
    Healthy,
    Poisoned,
}

#[derive(Component)]
pub enum Segment {
    Head {
        direction: Vec3,
        head_state: HeadState,
    },
    Body {
        parent: Option<Entity>,
    },
}

#[derive(Event)]
pub struct DespawnSegment {
    pub entity: Entity,
    pub direction: Option<Vec3>,
}

#[derive(Resource)]
pub struct SegmentSpawnerTimer(pub Timer);

impl Default for SegmentSpawnerTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(SEGMENT_SPAWN_TIMER_DURATION, TimerMode::Once);
        timer.pause();
        SegmentSpawnerTimer(timer)
    }
}

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
                        direction: Vec3::new(1.0, -1.0, 0.0),
                        head_state: HeadState::Healthy,
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

pub fn update_head_color(mut segment_query: Query<(&Segment, &mut Sprite)>) {
    for (segment, mut sprite) in segment_query.iter_mut() {
        match segment {
            Segment::Head {
                direction: _,
                head_state,
            } => {
                let color: Color;
                match head_state {
                    HeadState::Healthy => {
                        color = MILLIPEDE_HEAD_COLOR;
                    }
                    HeadState::Poisoned => {
                        color = MILLIPEDE_HEAD_COLOR_POISONED;
                    }
                }
                if sprite.color != color {
                    sprite.color = color;
                }
            }
            _ => continue,
        }
    }
}

// Stores the position in a global hashmap in order for the
// children segments to know the position of their parents
pub fn segment_movement(
    segment_positions: Res<SegmentPositions>,
    mut query: Query<(&Segment, &mut Transform)>,
    game_vars: Res<GameVariables>,
    time: Res<Time>,
) {
    for (segment, mut transform) in query.iter_mut() {
        match segment {
            Segment::Body { parent } => {
                if let Some(parent_entity) = parent {
                    if let Some(&parent_position) = segment_positions.0.get(parent_entity) {
                        let distance_to_parent = transform.translation.distance(parent_position);
                        if distance_to_parent > SEGMENT_SPACING {
                            let direction_to_parent =
                                (parent_position - transform.translation).normalize();
                            transform.translation += direction_to_parent
                                * game_vars.millipede_speed
                                * 3.0
                                * time.delta_seconds();

                            // Ensure that the segment doesn't move too close to its parent
                            if transform.translation.distance(parent_position) < SEGMENT_SPACING {
                                transform.translation =
                                    parent_position - direction_to_parent * SEGMENT_SPACING;
                            }

                            // Set the rotation to face the parent segment
                            let angle = direction_to_parent.y.atan2(direction_to_parent.x);
                            transform.rotation =
                                Quat::from_rotation_z(angle + std::f32::consts::FRAC_PI_2);
                        }
                    }
                }
            }
            Segment::Head {
                direction,
                head_state,
            } => {
                match head_state {
                    HeadState::Healthy => {
                        // Move in its direction
                        transform.translation.x +=
                            direction.x * time.delta_seconds() * game_vars.millipede_speed;

                        // Point the head in the direction it's heading
                        let target_angle = if direction.x > 0.0 {
                            -1.55 // Right
                        } else if direction.x < 0.0 {
                            1.55 // Left
                        } else if direction.y < 0.0 {
                            -std::f32::consts::FRAC_PI_2 // Down
                        } else {
                            transform.rotation.to_euler(EulerRot::XYZ).2 // Maintain current rotation
                        };

                        let current_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
                        let angle_diff = target_angle - current_angle;
                        let adjusted_angle =
                            current_angle + angle_diff * time.delta_seconds() * 100.0;
                        transform.rotation = Quat::from_rotation_z(adjusted_angle);
                    }
                    HeadState::Poisoned => {
                        // Move down
                        transform.translation.y -= time.delta_seconds() * game_vars.millipede_speed;

                        // Point the head downwards
                        let target_angle = -3.1;
                        let current_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
                        let angle_diff = target_angle - current_angle;
                        let adjusted_angle =
                            current_angle + angle_diff * time.delta_seconds() * 100.0;
                        transform.rotation = Quat::from_rotation_z(adjusted_angle);
                    }
                }
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
        if let Segment::Head {
            ref mut direction,
            head_state: _,
        } = *segment
        {
            // Check if hit bottom boundary
            if transform.translation.y < 5.0 + segment_radius {
                // Set direction to up
                direction.y = 1.0;
            }

            // Check if hit top player area boundary while in the up state
            if transform.translation.y > TOP_BOUND && direction.y == 1.0 {
                // Change back to down mode
                direction.y = -1.0;
            }

            // Check if hit left boundary
            if transform.translation.x < 0.0 + segment_radius {
                direction.x = 1.0;
                transform.translation.y += DROP_AMOUNT * direction.y;
            }

            // And right
            if transform.translation.x > window.width() - segment_radius {
                direction.x = -1.0;
                transform.translation.y += DROP_AMOUNT * direction.y;
            }
        }
    }
}

pub fn confine_segment_movement(
    mut head_query: Query<&mut Transform, With<Segment>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for mut transform in head_query.iter_mut() {
        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
        }
        if transform.translation.y > window.height() {
            transform.translation.y = window.height();
        }
        if transform.translation.x < 0.0 {
            transform.translation.x = 0.0;
        }
        if transform.translation.x > window.width() {
            transform.translation.x = window.width();
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
                if parent == Some(despawn_event.entity) {
                    *segment = Segment::Head {
                        direction: despawn_event
                            .direction
                            .unwrap_or_else(|| Vec3::new(1.0, -1.0, 0.0)),
                        head_state: HeadState::Healthy,
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
        if let Segment::Head {
            ref mut direction,
            head_state,
        } = *segment
        {
            if head_state == HeadState::Poisoned {
                return;
            }
            for shroom_transform in shroom_query.iter() {
                let distance = shroom_transform
                    .translation
                    .distance(segment_transform.translation);
                if distance < shroom_radius + segment_radius {
                    // Reverse direction
                    direction.x = -direction.x;
                    // Bounce backwards slightly
                    segment_transform.translation.x += direction.x * PUSH_BACK_AMOUNT;
                    // Move down
                    segment_transform.translation.y += DROP_AMOUNT * direction.y;
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
    mut down_timer: ResMut<DownTimer>,
    mut lives: ResMut<Lives>,
    mut explosion_events: EventWriter<ExplosionEvent>,
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
                    &mut commands,
                    &mut down_timer,
                    &mut lives,
                    &mut explosion_events,
                );
            }
        }
    }
}

pub fn spawn_lone_head(
    mut spawner_timer: ResMut<SegmentSpawnerTimer>,
    time: Res<Time>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    level_state: Res<State<LevelState>>,
) {
    // Only run if level not changing
    if let LevelState::Changing = level_state.get() {
        return;
    }

    spawner_timer.0.tick(time.delta());

    if spawner_timer.0.just_finished() {
        let starting_transform = Transform::from_xyz(0.0, TOP_BOUND, 0.0);
        // Spawn a head
        Millipede::spawn(1, &starting_transform, &mut commands, &game_assets);

        // Restart timer
        spawner_timer.0.reset();
    }
}

pub fn start_segment_spawner_timer(
    mut spawner_timer: ResMut<SegmentSpawnerTimer>,
    segment_query: Query<&Transform, With<Segment>>,
) {
    // Only run if the timer is paused
    if !spawner_timer.0.paused() {
        return;
    }
    for segment_transform in segment_query.iter() {
        if segment_transform.translation.y < TOP_BOUND * 0.2 {
            // A segment is below the threshold, unpause the timer
            spawner_timer.0.unpause();
        }
    }
}

// If a head collides with another head whilst on the same Y value, change their directions
pub fn collide_with_head(mut segment_query: Query<(Entity, &mut Transform, &mut Segment)>) {
    let mut heads = Vec::new();

    // Collect entities and their positions if they are heads
    for (entity, transform, segment) in segment_query.iter_mut() {
        if let Segment::Head {
            direction,
            head_state: _,
        } = &*segment
        {
            heads.push((entity, transform.translation, *direction));
        }
    }

    if heads.len() < 2 {
        // Don't run if there are fewer than 2 heads
        return;
    }

    // Collect changes to apply later
    let mut changes = Vec::new();

    // Compare each position
    for i in 0..heads.len() {
        for j in i + 1..heads.len() {
            let (entity1, pos1, _) = heads[i];
            let (entity2, pos2, _) = heads[j];

            if (pos1.y - pos2.y).abs() <= SEGMENT_SIZE / 1.3
                && (pos1.x - pos2.x).abs() <= SEGMENT_SIZE / 2.0
            {
                // Record the entities to change direction
                changes.push(entity1);
                changes.push(entity2);
            }
        }
    }

    // Apply direction changes
    for entity in changes {
        if let Ok((_, mut transform, mut segment)) = segment_query.get_mut(entity) {
            if let Segment::Head {
                direction,
                head_state: _,
            } = &mut *segment
            {
                direction.x = -direction.x;
                // Bounce backwards slightly
                let pushback = rand::thread_rng().gen_range(0..10);
                transform.translation.x += direction.x * PUSH_BACK_AMOUNT + (pushback as f32);

                // Randomly decide to drop
                // It needs to randomly drop in order to remove the chance that it gets caught in a
                // loop
                let drop = rand::thread_rng().gen_bool(SEGMENT_DROP_RATE);
                if drop {
                    transform.translation.y += DROP_AMOUNT * direction.y;
                }
            }
        }
    }
}

pub fn head_gets_poisoned(
    mut segment_query: Query<(&mut Segment, &Transform)>,
    mushroom_query: Query<(&Mushroom, &Transform)>,
) {
    // If a non-poisoned head touches a poison mushroom, the head
    // becomes poisoned
    let shroom_radius = MUSHROOM_SIZE / 3.0;
    let segment_radius = SEGMENT_SIZE / 3.0;
    for (mut segment, transform) in segment_query.iter_mut() {
        match &mut *segment {
            Segment::Head {
                direction: _,
                head_state,
            } => {
                match *head_state {
                    HeadState::Healthy => {
                        // Check if touching shroom
                        for (mushroom, mushroom_transform) in mushroom_query.iter() {
                            if *mushroom != Mushroom::Poison {
                                continue;
                            }

                            let distance = mushroom_transform
                                .translation
                                .distance(transform.translation);
                            if distance > shroom_radius + segment_radius {
                                continue;
                            }

                            *head_state = HeadState::Poisoned;
                        }
                    }
                    HeadState::Poisoned => {
                        // Check if hit bottom boundary
                        if transform.translation.y < 10.0 {
                            *head_state = HeadState::Healthy;
                        }
                    }
                }
            }
            _ => continue,
        }
    }
}
