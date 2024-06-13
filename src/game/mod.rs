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
pub mod scorpion;
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
use scorpion::*;
use shroom::*;
use spider::*;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (init_game, spawn_player, spawn_shroom_field).chain(),
        )
        .add_systems(
            OnExit(AppState::GameOver),
            (despawn_player, despawn_shroom_field),
        )
        .add_systems(
            Update,
            (
                (
                    (move_player, shoot_projectile, confine_player_movement)
                        .in_set(GameplaySet::Player),
                    (
                        move_projectile,
                        projectile_hits_scorpion,
                        projectile_hits_spider,
                        projectile_hits_segment,
                        projectile_hits_beetle,
                        projectile_hits_shroom,
                        despawn_projectile,
                        despawn_mushroom,
                    )
                        .in_set(GameplaySet::Projectile),
                    (
                        (
                            (spawn_shroom, spawn_beetle).chain(),
                            move_beetle,
                            beetle_spawn_shroom,
                            despawn_beetle,
                        ),
                        (
                            (
                                update_segment_parents,
                                update_positions,
                                segment_movement,
                                confine_segment_movement,
                                segment_hits_player,
                            ).chain(),
                            change_direction,
                            head_gets_poisoned,
                            collide_with_shroom,
                            start_segment_spawner_timer,
                            spawn_lone_head,
                            collide_with_head,
                            update_shroom_color,
                            update_head_color,
                            update_shroom_sprite,
                            animate_spider,
                            animate_segments,
                            animate_scorpion,
                        ),
                        (
                            spawn_spider,
                            set_spider_direction,
                            move_spider,
                            despawn_spider,
                            confine_spider_movement,
                            spider_hits_player,
                            spider_eats_shroom,
                            convert_to_poison_shroom,
                        ),
                        (spawn_scorpion, move_scorpion, despawn_scorpion),
                    )
                        .in_set(GameplaySet::Enemies),
                )
                    .run_if(in_state(GameState::Running))
                    .run_if(in_state(PlayerState::Alive)),
                (restart_level_from_death,)
                    .run_if(in_state(PlayerState::Dead))
                    .run_if(in_state(LevelState::Unchanging)),
                (
                    (start_new_level)
                        .chain()
                        .run_if(in_state(LevelState::Changing)),
                    (check_if_change_level).run_if(in_state(LevelState::Unchanging)),
                ),
                (heal_shrooms,).run_if(in_state(PlayerState::Dead)),
                ((
                    update_level_ui,
                    update_lives_ui,
                    update_score_ui,
                    spawn_explosion,
                    despawn_explosions,
                )),
                (animate_explosion,),
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, score_event)
        .insert_resource(SegmentPositions(HashMap::new()))
        .insert_resource(ShroomAmount(0))
        .insert_resource(Lives(STARTING_LIVES))
        .insert_resource(Score(0))
        .insert_resource(Level(0))
        .insert_resource(DownTimer(Timer::from_seconds(DOWNTIMER, TimerMode::Once)))
        .insert_resource(GameOverTimer(Timer::from_seconds(
            GAMEOVER_TIMER,
            TimerMode::Once,
        )))
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
        .add_event::<ExplosionEvent>()
        .add_event::<SpawnMushroomEvent>()
        .add_event::<FloatingScoreEvent>()
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
    Dead,
    Alive,
}

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct Health(pub i8);
