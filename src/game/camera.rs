use bevy::{prelude::*, window::PrimaryWindow, render::camera::ScalingMode};

#[derive(Component)]
pub struct GameCamera;

pub fn spawn_game_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        GameCamera,
    ));
}
