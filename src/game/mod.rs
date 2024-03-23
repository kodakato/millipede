use bevy::prelude::*;
use crate::AppState;

pub struct GamePlugin;

pub mod player;
pub mod camera;

use player::*;
use camera::*;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (spawn_player, spawn_game_camera));
    }
}
