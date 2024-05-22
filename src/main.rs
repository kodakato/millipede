use bevy::{app::AppExit, prelude::*};

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
        .add_systems(
            Update,
            (
                enter_game,
                ui::handle_button_actions,
                ui::handle_button_navigation,
                ui::update_button_colors,
            )
                .run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(Update, (toggle_pause).run_if(in_state(AppState::InGame)))
        .add_systems(Update, quit_game)
        .add_systems(Startup, (camera::spawn_game_camera).chain())
        .add_systems(OnEnter(AppState::MainMenu), ui::spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), ui::despawn_main_menu)
        .add_systems(OnEnter(AppState::InGame), ui::build_game_ui)
        .insert_resource(ui::SelectedButton(ui::ButtonType::Play))
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

fn quit_game(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if !input.pressed(constants::QUIT_KEY) {
        return;
    }
    exit.send(AppExit);
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
