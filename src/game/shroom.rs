use bevy::{prelude::*, window::PrimaryWindow};
use rand::*;

use crate::constants::*;

#[derive(Component)]
pub struct Mushroom;

pub fn spawn_shroom_field(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..MUSHROOM_MAX_AMOUNT {
        let x = rand::thread_rng().gen_range(0.0..window.width());
        let y = rand::thread_rng().gen_range(TOP_BOUND..window.height());

        let shroom_texture = asset_server.load("shroom.png");
        commands.spawn((
            SpriteBundle {
                texture: shroom_texture,
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Mushroom,
            Name::from("Mushroom"),
        ));
    }
}
