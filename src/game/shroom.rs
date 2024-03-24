use bevy::prelude::*;

#[derive(Component)]
pub struct Mushroom;

pub fn spawn_shroom_field(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shroom_texture = asset_server.load("shroom.png");
    commands.spawn((
        SpriteBundle {
            texture: shroom_texture,
            ..default()
        },
        Mushroom,
    ));
}
