use bevy::prelude::*;

use super::player::Player;
use crate::constants::*;

#[derive(Component)]
pub struct PlayerProjectile;

pub fn shoot_projectile(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    projectile_query: Query<Entity, With<PlayerProjectile>>,
    input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    // Check if a projectile already exists
    if !projectile_query.is_empty() {
        return;
    }

    // Check if shoot key is pressed
    if !(input.pressed(SHOOT_KEY)) {
        return;
    }
    if let Ok(player_transform) = player_query.get_single() {
        let projectile_texture = asset_server.load("stinger.png");
        let mut projectile_transform = *player_transform;
        projectile_transform.translation += Vec3::new(0.0, 6.0, 0.0);
        commands.spawn((
            Name::from("Player Projectile"),
            PlayerProjectile,
            SpriteBundle {
                texture: projectile_texture,
                transform: projectile_transform,
                ..default()
            },
        ));
    }
}

pub fn move_projectile(
    mut projectile_query: Query<&Transform, With<PlayerProjectile>>,
    ) {

}
