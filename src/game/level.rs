use bevy::{prelude::*, window::PrimaryWindow};

use super::{
    millipede::{Millipede, Segment},
    LevelState,
};
use crate::constants::*;

#[derive(Resource)]
pub struct Level(pub u32);

#[derive(Resource)]
pub struct DownTimer(pub Timer);

pub fn check_if_change_level(
    segment_query: Query<(), With<Segment>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut down_timer: ResMut<DownTimer>,
) {
    // Only run if all segments are dead
    if !segment_query.is_empty() {
        return;
    }

    // Start the down timer
    down_timer.0.reset();

    // All segments are dead, change the state
    next_level_state.set(LevelState::Changing);
}

pub fn start_new_level(
    mut timer: ResMut<DownTimer>,
    time: Res<Time>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    // Wait until the downtime is over
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    // Spawn new milipede
    Millipede::spawn(
        commands,
        asset_server,
        window_query,
        MILLIPEDE_STARTING_LENGTH,
    );

    // Reset to the unchanging level state
    next_level_state.set(LevelState::Unchanging);
}
