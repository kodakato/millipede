use bevy::prelude::*;

// Components
#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Spawn Player
    commands.spawn((
        Name::from("Player"),
        Player,
    ));
}

