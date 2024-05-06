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
    mut segment_spawner_timer: ResMut<SegmentSpawnerTimer>,
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
    game_vars.millipede_speed *= 1.01;

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

    // Pause and reset the segment spawner timer
    segment_spawner_timer.0.pause();
    segment_spawner_timer.0.reset();

    // Reset to the unchanging level state
    next_level_state.set(LevelState::Unchanging);
}

pub fn restart_level_from_death(
    mut commands: Commands,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    lives: ResMut<Lives>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut timer: ResMut<DownTimer>,
    time: Res<Time>,
    game_vars: Res<GameVariables>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_assets: Res<GameAssets>,
    segment_query: Query<Entity, With<Segment>>,
    spider_query: Query<Entity, With<Spider>>,
    spider_timer: ResMut<SpiderTimer>,
    mut segment_spawner_timer: ResMut<SegmentSpawnerTimer>,
) {
    if lives.0 == 0 {
        next_app_state.set(AppState::GameOver);
    }

    // Run down timer
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    // Despawn last millipede
    Millipede::despawn(&mut commands, &segment_query);

    // Despawn spider
    if let Ok(spider_entity) = spider_query.get_single() {
        Spider::despawn(spider_entity, &mut commands, spider_timer)
    }

    // Pause and reset the segment spawner timer
    segment_spawner_timer.0.pause();
    segment_spawner_timer.0.reset();

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

    // Spawn the player
    let starting_transform = Transform::from_xyz(window.width() / 2.0, PLAYER_SPAWN_Y, 0.0);
    Player::spawn(
        &starting_transform,
        &mut commands,
        &game_assets,
        &mut next_player_state,
    )
}
