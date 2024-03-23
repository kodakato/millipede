use bevy::prelude::*;

use crate::constants::*;

// Components
#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_model = asset_server.load("snake.png");
    // Spawn Player
    commands.spawn((
        SpriteBundle {
            texture: player_model,
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
