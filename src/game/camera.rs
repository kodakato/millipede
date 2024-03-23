use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera;

pub fn spawn_game_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        GameCamera,
    ));
}
