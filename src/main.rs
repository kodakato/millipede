use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

mod audio;
mod camera;
mod constants;
mod debug;
mod game;
mod ui;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Millipede".into(),
            canvas: Some("#game-canvas".into()),
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
        .add_plugins(AudioPlugin)
        .add_systems(
            Update,
            (
                ui::handle_button_actions,
                ui::handle_button_navigation,
                ui::update_button_colors,
            )
                .run_if(in_menu),
        )
        .add_systems(OnEnter(AppState::GameOver), (ui::spawn_game_over_ui,))
        .add_systems(OnExit(AppState::GameOver), (ui::despawn_game_over_ui,))
        .add_systems(
            OnEnter(AppState::MainMenu),
            ui::set_default_button_selection,
        )
        .add_systems(
            OnEnter(AppState::GameOver),
            ui::set_default_button_selection,
        )
        .add_systems(Update, (toggle_pause).run_if(in_state(AppState::InGame)))
        .add_systems(Startup, (camera::spawn_game_camera).chain())
        .add_systems(OnEnter(AppState::MainMenu), ui::spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), ui::despawn_main_menu)
        .add_systems(OnEnter(AppState::InGame), ui::build_game_ui)
        .insert_resource(ui::SelectedButton(ui::ButtonType::Play))
        .add_systems(Startup, (audio::prepare_audio).chain())
        .add_systems(Update, (audio::set_volume, audio::sync_audio))
        .run();
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

fn in_menu(state: Res<State<AppState>>) -> bool {
    if *state.get() == AppState::MainMenu || *state.get() == AppState::GameOver {
        true
    } else {
        false
    }
}
