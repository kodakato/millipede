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
    mut level: ResMut<Level>,
    mut game_vars: ResMut<GameVariables>,
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

    // Set millipede
    if game_vars.millipede_length < MILLIPEDE_MAX_LENGTH {
        game_vars.millipede_length += 1;
    }
    game_vars.millipede_speed *= 1.001;

    // Set Spider
    game_vars.spider_speed *= 1.001;
    game_vars.spider_attack_rate *= 1.001;
    game_vars.spider_leave_rate /= 1.01;

    if level.0 <= 5 {
        game_vars.spider_timer_length = SPIDER_TIMER;
    } else if level.0 <= 10 {
        game_vars.spider_timer_length = SPIDER_TIMER - 4.0;
        game_vars.spider_reward *= 2;
        game_vars.spider_average_spawn_height = SPIDER_AVERAGE_SPAWN_HEIGHT - 150.0;
    } else {
        game_vars.spider_timer_length = SPIDER_TIMER - 8.0;
        game_vars.spider_reward *= 2;
        game_vars.spider_average_spawn_height = SPIDER_AVERAGE_SPAWN_HEIGHT - 250.0;
    }

    // Spawn new milipede
    Millipede::spawn(
        game_vars.millipede_length,
        &starting_transform,
        &mut commands,
        &game_assets,
    );

    level.0 += 1;

    // Reset to the unchanging level state
    next_level_state.set(LevelState::Unchanging);
}

pub fn restart_level_from_death(
    mut commands: Commands,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut lives: ResMut<Lives>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut timer: ResMut<DownTimer>,
    time: Res<Time>,
    game_vars: Res<GameVariables>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_assets: Res<GameAssets>,
) {
    if lives.0 == 1 {
        next_app_state.set(AppState::GameOver);
    }

    // Run down timer
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    // Spawn millipede

    let window = window_query.get_single().unwrap();

    let x = window.width() / 2.0;
    let y = window.height() - TOP_UI_HEIGHT;

    let starting_transform = Transform::from_xyz(x, y, 0.0);

    Millipede::spawn(
        game_vars.millipede_length,
        &starting_transform,
        &mut commands,
        &game_assets,
    );

    // Sub the lives, and set player state
    lives.0 -= 1;
    next_player_state.set(PlayerState::Alive);
}

pub fn despawn_enemies(mut commands: Commands, segment_query: Query<Entity, With<Segment>>, spider_query: Query<Entity, With<Spider>>) {
    Millipede::despawn(&mut commands, &segment_query);
}
