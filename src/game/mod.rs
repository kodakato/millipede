use crate::AppState;
use bevy::{prelude::*, utils::HashMap};


pub struct GamePlugin;

pub mod camera;
pub mod millipede;
pub mod player;
pub mod projectile;
pub mod shroom;

use camera::*;
use millipede::*;
use player::*;
use projectile::*;
use shroom::*;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (
                spawn_game_camera,
                spawn_player,
                spawn_shroom_field,
                spawn_millipede,
            )
                .chain(),
        )
        .add_systems(
            Update,
            ((
                move_player,
                shoot_projectile,
                move_projectile,
                confine_player_movement,
                despawn_projectile,
                projectile_hits_shroom,
                segment_movement,
                update_positions,
            ))
                .run_if(in_state(AppState::InGame)),
        ).insert_resource(SegmentPositions(HashMap::new()));
    }
}
