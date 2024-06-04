// The beetle is responsible for spawning more shrooms
// if the current amount of shrooms goes below the threshold

use super::*;
use rand::*;

#[derive(Component)]
pub struct Beetle;

impl Beetle {
    pub fn spawn(
        starting_transform: &Transform,
        commands: &mut Commands,
        game_assets: &Res<GameAssets>,
    ) {
        let beetle_texture = &game_assets.beetle_texture;

        commands.spawn((
            Beetle,
            SpriteBundle {
                texture: beetle_texture.clone(),
                transform: *starting_transform,
                ..default()
            },
            Name::from("Beetle"),
        ));
    }
}

pub fn spawn_beetle(
    mut commands: Commands,
    shroom_amount: ResMut<ShroomAmount>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    game_assets: Res<GameAssets>,
    beetle_q: Query<&Beetle>,
) {
    // Check if under the threshold
    if shroom_amount.0 > MUSHROOM_MIN_AMOUNT {
        return;
    }

    // Check if beetle exists
    if !beetle_q.is_empty() {
        return;
    }

    let window = window_q.get_single().unwrap();
    // Generate a random starting position
    let x = rand::thread_rng().gen_range(0.0 + SPAWN_MARGIN..window.width() - SPAWN_MARGIN);
    let y = window.height();

    Beetle::spawn(&Transform::from_xyz(x, y, 0.0), &mut commands, &game_assets)
}

pub fn despawn_beetle(mut commands: Commands, beetle_q: Query<(Entity, &Transform), With<Beetle>>) {
    if let Ok((beetle_entity, beetle_transform)) = beetle_q.get_single() {
        // Check if below the bottom screen boundary
        if beetle_transform.translation.y < 0.0 {
            commands.entity(beetle_entity).despawn();
        }
    }
}

pub fn move_beetle(mut beetle_q: Query<&mut Transform, With<Beetle>>, time: Res<Time>) {
    if let Ok(mut beetle_transform) = beetle_q.get_single_mut() {
        beetle_transform.translation.y -= BEETLE_SPEED * time.delta_seconds();
    }
}

pub fn beetle_spawn_shroom(
    beetle_q: Query<&Transform, With<Beetle>>,
    mut spawn_mushroom_ew: EventWriter<SpawnMushroomEvent>,
) {
    if let Ok(beetle_transform) = beetle_q.get_single() {
        // Check if below boundary
        if beetle_transform.translation.y < TOP_BOUND {
            return;
        }

        // Generate a random num, and spawn if hit
        let num = rand::thread_rng().gen_range(1..=100);
        if num >= BEETLE_SPAWN_RATE {
            return;
        }

        let x = beetle_transform.translation.x;
        let y = beetle_transform.translation.y;

        spawn_mushroom_ew.send(SpawnMushroomEvent(Transform::from_xyz(x, y, 0.0)));
    }
}
