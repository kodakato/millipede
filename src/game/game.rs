use super::*;

#[derive(Resource)]
pub struct GameVariables {
    pub millipede_length: usize,
    pub millipede_speed: f32,
    pub spider_speed: f32,
    pub spider_attack_rate: f64,
    pub spider_leave_rate: f64,
    pub spider_timer_length: f32,
    pub spider_spawn_rate: f64,
    pub spider_average_spawn_height: f32,
    pub spider_reward: u32,
}

impl GameVariables {
    pub fn reset(&mut self) {
        self.millipede_length = MILLIPEDE_STARTING_LENGTH;
        self.millipede_speed = MILLIPEDE_SPEED;
        self.spider_speed = SPIDER_SPEED;
        self.spider_attack_rate = SPIDER_ATTACK_RATE;
        self.spider_leave_rate = SPIDER_LEAVE_RATE;
        self.spider_timer_length = SPIDER_TIMER;
        self.spider_spawn_rate = SPIDER_SPAWN_RATE;
        self.spider_average_spawn_height = SPIDER_AVERAGE_SPAWN_HEIGHT;
        self.spider_reward = SPIDER_REWARD;
    }
}

impl FromWorld for GameVariables {
    fn from_world(_world: &mut World) -> Self {
        let mut vars = GameVariables {
            millipede_length: MILLIPEDE_STARTING_LENGTH,
            millipede_speed: MILLIPEDE_SPEED,
            spider_speed: SPIDER_SPEED,
            spider_attack_rate: SPIDER_ATTACK_RATE,
            spider_leave_rate: SPIDER_LEAVE_RATE,
            spider_timer_length: SPIDER_TIMER,
            spider_spawn_rate: SPIDER_SPAWN_RATE,
            spider_average_spawn_height: SPIDER_AVERAGE_SPAWN_HEIGHT,
            spider_reward: SPIDER_REWARD,
        };
        vars.reset();
        vars
    }
}

pub fn init_game(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    mut score: ResMut<Score>,
    mut level: ResMut<Level>,
    mut down_timer: ResMut<DownTimer>,
    mut spider_timer: ResMut<SpiderTimer>,
    player_query: Query<Entity, With<Player>>,
    mushroom_query: Query<Entity, With<Mushroom>>,
    millipede_query: Query<Entity, With<Segment>>,
    spider_query: Query<Entity, With<Spider>>,
    scorpion_query: Query<Entity, With<Scorpion>>,
    beetle_query: Query<Entity, With<Beetle>>,
    explosion_query: Query<Entity, With<Explosion>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut shroom_amount: ResMut<ShroomAmount>,
    mut game_vars: ResMut<GameVariables>,
) {
    lives.0 = STARTING_LIVES;

    score.0 = 0;

    level.0 = 0;

    // Reset Game vars
    game_vars.reset();

    down_timer.0.reset();

    // Depspawn existing spider
    if let Ok(spider_entity) = spider_query.get_single() {
        commands.entity(spider_entity).despawn();
    }

    spider_timer.0.reset();

    // Despawn existing scorpion
    if let Ok(scorpion_entity) = scorpion_query.get_single() {
        commands.entity(scorpion_entity).despawn();
    }

    // Despawn existing beetle
    if let Ok(beetle_entity) = beetle_query.get_single() {
        commands.entity(beetle_entity).despawn();
    }

    // Despawn existing player
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }

    // Despawn existing shrooms
    for mushroom_entity in mushroom_query.iter() {
        commands.entity(mushroom_entity).despawn();
    }
    shroom_amount.0 = 0;

    // Despawn existing millipede
    for millipede_entity in millipede_query.iter() {
        commands.entity(millipede_entity).despawn();
    }

    // Despawn existing explosions
    for explosion_entity in explosion_query.iter() {
        commands.entity(explosion_entity).despawn();
    }
    // Init level state
    next_level_state.set(LevelState::Changing);
}
