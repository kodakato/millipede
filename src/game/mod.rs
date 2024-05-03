use crate::{AppState, GameState};
use bevy::{prelude::*, utils::HashMap, window::PrimaryWindow};

pub struct GamePlugin;

pub mod assets;
pub mod beetle;
pub mod explosion;
pub mod game;
pub mod level;
pub mod millipede;
pub mod player;
pub mod projectile;
pub mod shroom;
pub mod spider;

use crate::{constants::*, ui::*};
use assets::*;
use beetle::*;
use explosion::*;
use game::*;
use level::*;
use millipede::*;
use player::*;
use projectile::*;
use shroom::*;
use spider::*;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (spawn_player, spawn_shroom_field).chain(),
        )
        .add_systems(
            Update,
            (
                (
                    (move_player, shoot_projectile, confine_player_movement)
                        .in_set(GameplaySet::Player),
                    (
                        move_projectile,
                        projectile_hits_spider,
                        projectile_hits_segment,
                        projectile_hits_beetle,
                        projectile_hits_shroom,
                        despawn_projectile,
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
                            start_segment_spawner_timer,
                            spawn_lone_head,
                        )
                            .chain(),
                        (
                            spawn_spider,
                            set_spider_direction,
                            move_spider,
                            despawn_spider,
                            confine_spider_movement,
                            spider_hits_player,
                        )
                            .chain(),
                    )
                        .in_set(GameplaySet::Enemies),
                    ((check_if_change_level,)).run_if(in_state(LevelState::Unchanging)),
                    ((start_new_level).chain()).run_if(in_state(LevelState::Changing)),
                )
                    .run_if(in_state(GameState::Running))
                    .run_if(in_state(PlayerState::Alive)),
                ((restart_level_from_death).chain()).run_if(in_state(PlayerState::Dead)),
                ((update_level_ui, update_lives_ui, update_score_ui, despawn_explosions,)),
            )
                .run_if(in_state(AppState::InGame)),
        )
        .insert_resource(SegmentPositions(HashMap::new()))
        .insert_resource(ShroomAmount(0))
        .insert_resource(Lives(STARTING_LIVES))
        .insert_resource(Score(0))
        .insert_resource(Level(0))
        .insert_resource(DownTimer(Timer::from_seconds(DOWNTIMER, TimerMode::Once)))
        .insert_resource(SpiderTimer(Timer::from_seconds(
            SPIDER_TIMER,
            TimerMode::Once,
        )))
        .init_resource::<GameAssets>()
        .init_resource::<GameVariables>()
        .init_resource::<SegmentSpawnerTimer>()
        .configure_sets(
            Update,
            (
                GameplaySet::Player.before(GameplaySet::Projectile),
                GameplaySet::Projectile.before(GameplaySet::Enemies),
                GameplaySet::Enemies,
            ),
        )
        .add_event::<DespawnSegment>()
        .init_state::<LevelState>()
        .init_state::<PlayerState>();
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

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum LevelState {
    #[default]
    Changing,
    Unchanging,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum PlayerState {
    #[default]
    Alive,
    Dead,
}

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct Health(pub i8);
