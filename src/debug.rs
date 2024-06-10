use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::env;

use crate::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            // World inspector plugin
            let inspector_enabled = env::args().any(|arg| arg == "--inspector");
            if inspector_enabled {
                app.add_plugins(WorldInspectorPlugin::new());
            }
            app.add_systems(
                Update,
                kill_player.run_if(in_state(crate::game::PlayerState::Alive)),
            )
            .add_systems(Update, toggle_pause.run_if(in_state(AppState::InGame)));
        }
    }
}

fn toggle_pause(
    state: Res<State<crate::GameState>>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(constants::MENU_KEY) {
        match state.get() {
            GameState::Running => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Running),
        }
    }
}

fn kill_player(
    mut next_state: ResMut<NextState<crate::game::PlayerState>>,
    input: Res<ButtonInput<KeyCode>>,
    player_q: Query<Entity, With<crate::game::player::Player>>,
    mut commands: Commands,
    mut down_timer: ResMut<crate::game::level::DownTimer>,
    mut lives: ResMut<crate::game::player::Lives>,
    mut event: EventWriter<crate::game::explosion::ExplosionEvent>,
) {
    if input.just_released(KeyCode::KeyK) {
        let entity = player_q.get_single().unwrap();
        crate::game::player::Player::kill(
            &Transform::from_xyz(100.0, 100.0, 0.0),
            entity,
            &mut next_state,
            &mut commands,
            &mut down_timer,
            &mut lives,
            &mut event,
        )
    }
}
