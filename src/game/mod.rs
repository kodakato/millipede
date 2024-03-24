use crate::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

pub mod camera;
pub mod player;
pub mod projectile;
pub mod shroom;

use camera::*;
use player::*;
use projectile::*;
use shroom::*;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (spawn_player, spawn_game_camera, spawn_shroom_field),
        )
        .add_systems(
            Update,
            ((move_player, shoot_projectile, move_projectile)).run_if(in_state(AppState::InGame)),
        );
    }
}
