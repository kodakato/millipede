use super::*;
use rand::*;

use crate::constants::*;

#[derive(Component)]
pub struct Mushroom;

impl Mushroom {
    pub fn spawn(location_transform: &Transform, commands: &mut Commands, game_assets: &Res<GameAssets>, shroom_amount: &mut ResMut<ShroomAmount>) {
        let shroom_texture = &game_assets.shroom_texture;

        commands.spawn((
                Mushroom,
                Health(MUSHROOM_HEALTH),
                SpriteBundle {
                    texture: shroom_texture.clone(),
                    transform: *location_transform,
                    ..default()
                },
                Name::from("Mushroom"),
        ));

        // Add to shroom count
        shroom_amount.0 += 1;
    }
}

// Resources

#[derive(Resource)]
pub struct ShroomAmount(pub u8);


pub fn spawn_shroom_field(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut shroom_amount: ResMut<ShroomAmount>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..MUSHROOM_MAX_AMOUNT {
        let x = rand::thread_rng().gen_range(0.0 + SPAWN_MARGIN..window.width() - SPAWN_MARGIN);
        let y = rand::thread_rng().gen_range(TOP_BOUND..window.height() - TOP_UI_HEIGHT);

        Mushroom::spawn(&Transform::from_xyz(x, y, 0.0), &mut commands, &game_assets, &mut shroom_amount);
    }
}

pub fn despawn_mushroom(
    mut commands: Commands,
    shroom_q: Query<(Entity, &Health)>,
    mut shroom_amount: ResMut<ShroomAmount>,
) {
    for (shroom_entity, shroom_health) in shroom_q.iter() {
        // Skip those with health
        if shroom_health.0 > 0 {
            continue;
        }
        commands.entity(shroom_entity).despawn();
        shroom_amount.0 -= 1;
    }
}
