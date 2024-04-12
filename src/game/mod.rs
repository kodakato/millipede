use crate::{AppState, GameState};
use bevy::{prelude::*, utils::HashMap};

pub struct GamePlugin;

pub mod beetle;
pub mod explosion;
pub mod millipede;
pub mod player;
pub mod projectile;
pub mod shroom;

use beetle::*;
use explosion::*;
use millipede::*;
use player::*;
use projectile::*;
use shroom::*;
use crate::ui::*;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (
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
                    projectile_hits_segment,
                    projectile_hits_beetle,
                    projectile_hits_shroom,
                    despawn_projectile,
                    despawn_explosions,
                    despawn_mushroom,
                )
                    .in_set(GameplaySet::Projectile)
                    .chain(),
                (
                    (
                        spawn_beetle,
                        move_beetle,
                        beetle_spawn_shroom,
                        despawn_beetle,
                    )
                        .chain(),
                    (
                        update_segment_parents,
                        update_positions,
                        segment_movement,
                        change_direction,
                        collide_with_shroom,
                        segment_hits_player,
                    )
                        .chain(),
                )
                    .in_set(GameplaySet::Enemies),
                (
                    (
                        update_game_ui
                    )
               )
                    .in_set(GameplaySet::Ui)
            ),)
                .run_if(in_state(GameState::Running))
                .run_if(in_state(AppState::InGame)),
        )
        .insert_resource(SegmentPositions(HashMap::new()))
        .insert_resource(ShroomAmount(0))
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
    Ui
}
