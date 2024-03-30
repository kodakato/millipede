use crate::{AppState, GameState};
use bevy::{prelude::*, utils::HashMap};

pub struct GamePlugin;

pub mod camera;
pub mod explosion;
pub mod millipede;
pub mod player;
pub mod projectile;
pub mod shroom;

use camera::*;
use explosion::*;
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
                (move_player, shoot_projectile, confine_player_movement)
                    .in_set(GameplaySet::Player),
                (
                    move_projectile,
                    despawn_projectile,
                    projectile_hits_shroom,
                    projectile_hits_segment,
                    despawn_explosions,
                )
                    .in_set(GameplaySet::Projectile)
                    .chain(),
                (
                    update_segment_parents,
                    update_positions,
                    segment_movement,
                    change_direction,
                    collide_with_shroom,
                    segment_hits_player,
                )
                    .chain()
                    .in_set(GameplaySet::Enemies),
            ),)
                .run_if(in_state(GameState::Running))
                .run_if(in_state(AppState::InGame)),
        )
        .insert_resource(SegmentPositions(HashMap::new()))
        .configure_sets(
            Update,
            (
                GameplaySet::Player.before(GameplaySet::Projectile),
                GameplaySet::Projectile.before(GameplaySet::Enemies),
                GameplaySet::Enemies,
            ),
        )
        .add_event::<DespawnSegment>();
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplaySet {
    Player,
    Enemies,
    Projectile,
}
