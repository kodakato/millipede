use super::*;
use rand::Rng;

pub enum SpiderState {
    Centering,
    Attacking,
    Wandering,
    Leaving,
}

#[derive(Component)]
pub struct Spider(SpiderState);

#[derive(Component)]
pub struct Direction(pub Vec3);

// ## Behaviour
// Spider should spawn, run to the center, and then spend a random amount of time in the play area,
// going in random directions
impl Spider {
    pub fn spawn(
        location_transform: &Transform,
        direction: &Vec3,
        commands: &mut Commands,
        game_assets: &Res<GameAssets>,
    ) {
        let spider_texture = &game_assets.spider_texture;

        commands.spawn((
            SpriteBundle {
                texture: spider_texture.clone(),
                transform: *location_transform,
                ..default()
            },
            Spider(SpiderState::Centering),
            Direction(*direction),
        ));
    }

    pub fn despawn(entity: Entity, commands: &mut Commands, mut spider_timer: ResMut<SpiderTimer>) {
        commands.entity(entity).despawn();

        spider_timer.0.reset();
    }
}

#[derive(Resource)]
pub struct SpiderTimer(pub Timer);

pub fn spawn_spider(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut spider_timer: ResMut<SpiderTimer>,
    time: Res<Time>,
    spider_query: Query<(), With<Spider>>,
    game_vars: Res<GameVariables>,
) {
    // Only run if no current spider
    if !spider_query.is_empty() {
        return;
    }

    // Tick and check timer, only run when timer is up
    spider_timer.0.tick(time.delta());
    if !spider_timer.0.finished() {
        return;
    }

    // Generate a random num to determine if the spider should spawn
    let run = rand::thread_rng().gen_bool(SPIDER_SPAWN_RATE);
    if !run {
        return;
    }

    let window = window_query.get_single().unwrap();

    let mut x_start = 0.0;
    let mut direction = Vec3::X;
    let mut y_start = game_vars.spider_average_spawn_height;

    let random_side = rand::thread_rng().gen_bool(0.5);

    if random_side {
        x_start = window.width();
        direction = -Vec3::X;
    }; // Switch to the right side 50% of the time

    y_start = rand::thread_rng().gen_range(y_start - 50.0..50.0 + y_start);

    let location_transform = Transform::from_xyz(x_start, y_start, 0.0);
    Spider::spawn(&location_transform, &direction, &mut commands, &game_assets)
}

pub fn set_spider_direction(
    mut spider_query: Query<(&Transform, &mut Direction, &mut Spider)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    game_vars: Res<GameVariables>,
) {
    // Only run if spider exists
    if let Ok((transform, mut direction, mut spider)) = spider_query.get_single_mut() {
        match spider.0 {
            SpiderState::Centering => {
                let window = window_query.get_single().unwrap();

                // If on right of center, direction goes left, and vice versa. Check if on center.
                // I need to check between a threshold because being exactly on center is difficult
                // to calculate
                if transform.translation.x >= window.width() / 2.0 - 2.0
                    && transform.translation.x <= window.width() / 2.0 + 2.0
                {
                    spider.0 = SpiderState::Wandering;
                } else if transform.translation.x > window.width() / 2.0 {
                    direction.0 = -Vec3::X; //Left
                } else {
                    direction.0 = Vec3::X; // Right
                }

                let y = rand::thread_rng().gen_range(-0.25..0.25);
                direction.0.y = y;
            }
            SpiderState::Wandering => {
                // Only change direction by random chance
                let run = rand::thread_rng().gen_bool(SPIDER_DIRECTION_CHANGE_RATE);
                if !run {
                    return;
                }

                let x = rand::thread_rng().gen_range(-1.0..1.0);
                let y = rand::thread_rng().gen_range(-1.0..1.0);

                let new_direction = Vec3::new(x, y, 0.0).normalize_or_zero();
                direction.0 = new_direction;

                // Randomly set to attack
                let attack = rand::thread_rng().gen_bool(game_vars.spider_attack_rate);
                if !attack {
                    return;
                }
                // Get player's position, set course for it
                if let Ok(player_transform) = player_query.get_single() {
                    let direction_to_player_x = (player_transform.translation
                        - transform.translation)
                        .normalize()
                        .x;

                    // Set the spider direction to it
                    direction.0.x = direction_to_player_x;
                    direction.0.y = -1.0;

                    // Now set the state to attack
                    spider.0 = SpiderState::Attacking;
                }
            }
            SpiderState::Attacking => {
                // Once the spider hits the bottom, it switch back to wandering mode or leave
                if transform.translation.y <= 0.0 {
                    // Randomly decide to leave
                    let leave = rand::thread_rng().gen_bool(game_vars.spider_leave_rate);
                    if leave {
                        spider.0 = SpiderState::Leaving;
                    } else {
                        spider.0 = SpiderState::Wandering;
                    }
                }
            }
            SpiderState::Leaving => {
                if !(direction.0.x == -1.0 || direction.0.x == 1.0) {
                    let right = rand::thread_rng().gen_bool(0.5);
                    if right {
                        direction.0.x = 1.0;
                    } else {
                        direction.0.x = -1.0;
                    }
                    direction.0.y = 1.0;
                }
            }
        }
    }
}

pub fn move_spider(
    mut spider_query: Query<(&mut Transform, &mut Direction)>,
    time: Res<Time>,
    game_vars: Res<GameVariables>,
) {
    // Only run if a spider exists
    if let Ok((mut spider_transform, direction)) = spider_query.get_single_mut() {
        // Move in direction
        spider_transform.translation.x +=
            direction.0.x * game_vars.spider_speed * time.delta_seconds();
        spider_transform.translation.y +=
            direction.0.y * game_vars.spider_speed * time.delta_seconds() * 1.5;
    }
}

pub fn confine_spider_movement(
    mut spider_query: Query<(&mut Transform, &mut Direction, &Spider)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Run only if spider exists
    if let Ok((mut transform, mut direction, spider)) = spider_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let upper_bound = window.height() - TOP_UI_HEIGHT;
        let lower_bound = 0.0;
        let left_bound = 0.0;
        let right_bound = window.width();

        match spider.0 {
            SpiderState::Leaving => {}
            _ => {
                if transform.translation.x < left_bound {
                    transform.translation.x = left_bound;
                    direction.0.x = -direction.0.x;
                }

                if transform.translation.x > right_bound {
                    transform.translation.x = right_bound;
                    direction.0.x = -direction.0.x;
                }
            }
        }

        if transform.translation.y > upper_bound {
            transform.translation.y = upper_bound;
            direction.0.y = -direction.0.y;
        }

        if transform.translation.y < lower_bound {
            transform.translation.y = lower_bound;
            direction.0.y = -direction.0.y;
        }
    }
}

pub fn despawn_spider(
    spider_query: Query<(&Transform, Entity, &Spider)>,
    spider_timer: ResMut<SpiderTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    if let Ok((transform, entity, spider)) = spider_query.get_single() {
        match spider.0 {
            SpiderState::Leaving => {
                let window = window_query.get_single().unwrap();

                if transform.translation.x < 0.0 || transform.translation.y > window.width() {
                    Spider::despawn(entity, &mut commands, spider_timer);
                }
            }
            _ => {}
        }
    }
}

pub fn spider_hits_player(
    spider_query: Query<(&Transform), With<Spider>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    game_assets: Res<GameAssets>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut commands: Commands,
    mut down_timer: ResMut<DownTimer>,
    mut lives: ResMut<Lives>,
) {
    if let Ok(spider_transform) = spider_query.get_single() {
        if let Ok((player_entity, player_transform)) = player_query.get_single() {
            let spider_radius = SPIDER_SIZE / 2.0;
            let player_radius = PLAYER_SIZE / 2.0;

            let distance = spider_transform
                .translation
                .distance(player_transform.translation);
            if distance > spider_radius + player_radius {
                return;
            }
            Player::kill(&player_transform, player_entity, &mut next_player_state, &game_assets, &mut commands, &mut down_timer, &mut lives)
        }
    }
}
