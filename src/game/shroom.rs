use bevy::{prelude::*, window::PrimaryWindow};
use rand::*;

use crate::constants::*;

#[derive(Component)]
pub struct Mushroom;

impl Mushroom {
    pub fn bundle(
        x: f32,
        y: f32,
        asset_server: &Res<AssetServer>,
        shroom_amount: &mut ResMut<ShroomAmount>,
    ) -> (SpriteBundle, Self, Name, Health) {
        shroom_amount.0 += 1;

        let shroom_texture: Handle<Image> = asset_server.load("shroom.png");
        (
            SpriteBundle {
                texture: shroom_texture,
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Mushroom,
            Name::from("Mushroom"),
            Health(MUSHROOM_HEALTH),
        )
    }
}

// Resources

#[derive(Resource)]
pub struct ShroomAmount(pub u8);

#[derive(Component)]
pub struct Health(pub i8);

pub fn spawn_shroom_field(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut shroom_amount: ResMut<ShroomAmount>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..MUSHROOM_MAX_AMOUNT {
        let x = rand::thread_rng().gen_range(0.0 + SPAWN_MARGIN..window.width() - SPAWN_MARGIN);
        let y = rand::thread_rng().gen_range(TOP_BOUND..window.height() - TOP_UI_HEIGHT);

        commands.spawn(Mushroom::bundle(x, y, &asset_server, &mut shroom_amount));
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
