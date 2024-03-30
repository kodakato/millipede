use bevy::prelude::*;

mod constants;
mod debug;
mod game;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Millipede".into(),
            resolution: (480., 640.).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    };
    App::new()
        .init_state::<AppState>()
        .init_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(window_plugin),
        )
        .add_plugins(debug::DebugPlugin)
        .add_plugins(game::GamePlugin)
        .add_systems(Update, (enter_game).run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, (toggle_pause).run_if(in_state(AppState::InGame)))
        .run();
}

fn enter_game(mut next_state: ResMut<NextState<AppState>>, input: Res<ButtonInput<KeyCode>>) {
    // Check if player hits play button
    if input.just_pressed(constants::MENU_KEY) {
        next_state.set(AppState::InGame)
    }
}

fn toggle_pause(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(constants::MENU_KEY) {
        match state.get() {
            GameState::Running => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Running),
        }
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}
