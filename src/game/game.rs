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

impl FromWorld for GameVariables {
    fn from_world(_world: &mut World) -> Self {
        GameVariables {
            millipede_length: MILLIPEDE_STARTING_LENGTH,
            millipede_speed: MILLIPEDE_SPEED,
            spider_speed: SPIDER_SPEED,
            spider_attack_rate: SPIDER_ATTACK_RATE,
            spider_leave_rate: SPIDER_LEAVE_RATE,
            spider_timer_length: SPIDER_TIMER,
            spider_spawn_rate: SPIDER_SPAWN_RATE,
            spider_average_spawn_height: SPIDER_AVERAGE_SPAWN_HEIGHT,
            spider_reward: SPIDER_REWARD,
        }
    }
}
