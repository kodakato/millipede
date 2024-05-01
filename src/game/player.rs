use super::*;

// Components
#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(
        location_transform: &Transform,
        commands: &mut Commands,
        game_assets: &Res<GameAssets>,
        next_player_state: &mut ResMut<NextState<PlayerState>>,
    ) {
        let player_texture = &game_assets.player_texture;

        // Spawn Player
        commands.spawn((
            SpriteBundle {
                texture: player_texture.clone(),
                transform: *location_transform,
                ..default()
            },
            Name::from("Player"),
            Player,
        ));

        // Set player state
        next_player_state.set(PlayerState::Alive);
    }

    pub fn kill(
        player_transform: &Transform,
        player_entity: Entity,
        next_player_state: &mut ResMut<NextState<PlayerState>>,
        game_assets: &Res<GameAssets>,
        mut commands: &mut Commands,
        down_timer: &mut ResMut<DownTimer>,
        lives: &mut ResMut<Lives>,
    ) {
        // Despawn player
        commands.entity(player_entity).despawn();
        // Spawn explosion
        Explosion::spawn(&player_transform, &mut commands, &game_assets);

        // Set next state
        next_player_state.set(PlayerState::Dead);

        // Subtract lives
        lives.0 -= 1;

        // Start down timer
        down_timer.0.reset();
    }
}

#[derive(Resource)]
pub struct Lives(pub u8);

pub fn spawn_player(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    let window = window_query.get_single().unwrap();

    let starting_transform = Transform::from_xyz(window.width() / 2.0, PLAYER_SPAWN_Y, 0.0);

    Player::spawn(
        &starting_transform,
        &mut commands,
        &game_assets,
        &mut next_player_state,
    );
}

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if input.pressed(LEFT) {
            player_transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(RIGHT) {
            player_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(UP) {
            player_transform.translation.y += PLAYER_SPEED * time.delta_seconds();
        }
        if input.pressed(DOWN) {
            player_transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = TOP_BOUND - half_player_size;

        let mut translation = player_transform.translation;

        // Bound x
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound y
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
