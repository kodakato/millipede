use bevy::{prelude::*, window::PrimaryWindow};

use crate::constants::*;

// Components
#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let player_model = asset_server.load("snake.png");
    // Spawn Player
    commands.spawn((
        SpriteBundle {
            texture: player_model,
            transform: Transform::from_xyz(window.width() / 2.0, 20.0, 0.0),
            ..default()
        },
        Name::from("Player"),
        Player,
    ));
}

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if input.pressed(LEFT) {
            player_transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(RIGHT) {
            player_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(UP) {
            player_transform.translation.y += PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(DOWN) {
            player_transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        
        
    }
}
