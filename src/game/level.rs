use super::*;

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
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    // Wait until the downtime is over
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    let window = window_query.get_single().unwrap();

    let x = window.width() / 2.0;
    let y = window.height() - TOP_UI_HEIGHT;
    
    let starting_transform = Transform::from_xyz(x, y, 0.0);
    
    // Spawn new milipede
    Millipede::spawn(
        MILLIPEDE_STARTING_LENGTH,
        &starting_transform,
        &mut commands,
        &game_assets,
    );

    // Reset to the unchanging level state
    next_level_state.set(LevelState::Unchanging);
}
