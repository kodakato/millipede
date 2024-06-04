use super::*;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Scorpion {
    direction: Vec3,
}

impl Scorpion {
    pub fn spawn(
        starting_transform: &Transform,
        commands: &mut Commands,
        game_assets: &Res<GameAssets>,
    ) {
        let scorpion_texture = &game_assets.scorpion_texture;

        let mut direction = Vec3::new(1.0, 0.0, 0.0); // Default moving right
                                                      // Check if it starts on the right, and set the direction to left
        if starting_transform.translation.x > 2.0 {
            direction.x = -direction.x;
        }

        commands.spawn((
            Scorpion { direction },
            SpriteBundle {
                texture: scorpion_texture.clone(),
                transform: *starting_transform,
                ..default()
            },
            Name::from("Scorpion"),
        ));
    }

    pub fn despawn(scorpion_entity: Entity, commands: &mut Commands) {
        commands.entity(scorpion_entity).despawn();
    }

    pub fn kill(scorpion_entity: Entity, commands: &mut Commands, score: &mut ResMut<Score>) {
        Self::despawn(scorpion_entity, commands);
        score.0 += SCORPION_REWARD;
    }
}

pub fn spawn_scorpion(
    mut commands: Commands,
    level: Res<Level>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_assets: Res<GameAssets>,
    scorpion_query: Query<(), With<Scorpion>>,
) {
    // Only run if above a certain level
    if level.0 < 0 {
        return;
    }

    // Don't run if a scorpion already exists
    if !scorpion_query.is_empty() {
        return;
    }

    // Only spawn a certain percentage of the time
    let spawn = rand::thread_rng().gen_bool(SCORPION_SPAWN_RATE);
    if !spawn {
        return;
    }

    // Spawn scorpion
    let mut starting_transform: Transform = Transform::default();

    let window = window_query.get_single().unwrap();

    // Choose either left or right
    let left = rand::thread_rng().gen_bool(0.5);
    if left {
        starting_transform.translation.x = 1.0;
        starting_transform.rotation = Quat::from_rotation_z(-1.55);
    } else {
        starting_transform.translation.x = window.width() - 1.0;
        starting_transform.rotation = Quat::from_rotation_z(1.55);
    }

    // Now set height
    let height_range = rand::thread_rng().gen_range(-75.0..75.0);
    starting_transform.translation.y = window.height() - SCORPION_SPAWN_HEIGHT + height_range;

    Scorpion::spawn(&starting_transform, &mut commands, &game_assets);
}

pub fn move_scorpion(mut scorpion_query: Query<(&mut Transform, &Scorpion)>, time: Res<Time>) {
    if let Ok((mut scorpion_transform, scorpion)) = scorpion_query.get_single_mut() {
        scorpion_transform.translation.x +=
            scorpion.direction.x * time.delta_seconds() * SCORPION_SPEED;
    }
}

pub fn despawn_scorpion(
    scorpion_query: Query<(&Transform, Entity), With<Scorpion>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    if let Ok((scorpion_transform, scorpion_entity)) = scorpion_query.get_single() {
        let window = window_query.get_single().unwrap();

        // Despawn if outside the window
        if scorpion_transform.translation.x < 0.0
            || scorpion_transform.translation.x > window.width()
        {
            Scorpion::despawn(scorpion_entity, &mut commands);
        }
    }
}

pub fn convert_to_poison_shroom(
    scorpion_query: Query<&Transform, With<Scorpion>>,
    mut mushroom_query: Query<(&Transform, &mut Mushroom)>,
) {
    for scorpion_transform in scorpion_query.iter() {
        for (mushroom_transform, mut mushroom) in mushroom_query.iter_mut() {
            // Check if scorpion collides with a shroom
            let mushroom_radius = MUSHROOM_SIZE / 2.0;
            let scorpion_radius = SCORPION_SIZE / 2.0;
            let distance = scorpion_transform
                .translation
                .distance(mushroom_transform.translation);
            if distance <= mushroom_radius + scorpion_radius {
                // Randomly choose to turn it into poison mushroom
                if *mushroom == Mushroom::Poison {
                    return;
                }
                let convert = rand::thread_rng().gen_bool(MUSHROOM_CONVERSION_RATE);
                if !convert {
                    return;
                }
                *mushroom = Mushroom::Poison;
            }
        }
    }
}
