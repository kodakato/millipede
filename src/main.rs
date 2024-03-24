use bevy::prelude::*;

mod constants;
mod debug;
mod game;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Millipede".into(),
            resolution: (480., 640.).into(),
            ..default()
        }),
        ..default()
    };
    App::new()
        .init_state::<AppState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(window_plugin),
        )
        .add_plugins(debug::DebugPlugin)
        .add_plugins(game::GamePlugin)
        .add_systems(Update, (enter_game).run_if(in_state(AppState::MainMenu)))
        .run();
}

fn enter_game(mut next_state: ResMut<NextState<AppState>>, input: Res<ButtonInput<KeyCode>>) {
    // Check if player hits play button
    if input.just_pressed(constants::MENU_KEY) {
        next_state.set(AppState::InGame)
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
