use crate::{
    constants::*,
    game::{assets::*, level::Level, player::Lives, Score},
};

use super::*; 
use bevy::{app::AppExit, prelude::*};

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Resource)]
pub struct SelectedButton(pub ButtonType);

#[derive(Clone, Copy, Component, PartialEq)]
pub enum ButtonType {
    Play,
    Quit,
    Restart,
    MainMenu,
}

// Main Menu
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Define the main menu parent node with MainMenu marker struct
    let main_menu_node = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        },
        MainMenu,
    );

    // Spawn parent entity and attach the main menu component
    let main_menu_entity = commands.spawn(main_menu_node).id();

    // Define the title node
    let title_node = NodeBundle {
        style: Style {
            height: Val::Percent(20.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    };

    let title_entity = commands
        .spawn(title_node)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Millipede!",
                        TextStyle {
                            color: Color::GREEN.into(),
                            font_size: 60.0,
                            ..default()
                        },
                    )],
                    ..default()
                },
                ..default()
            });
        })
        .id();

    // Define the button node
    let buttons_node = NodeBundle {
        style: Style {
            height: Val::Percent(60.0),
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        },
        ..default()
    };

    let buttons_entity = commands
        .spawn(buttons_node)
        .with_children(|parent| {
            // Play Button
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Play",
                            TextStyle {
                                color: Color::GREEN.into(),
                                font_size: 40.0,
                                ..default()
                            },
                        )],
                        ..default()
                    },
                    ..default()
                },
                ButtonType::Play,
            ));

            // Quit Button
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Quit",
                            TextStyle {
                                color: Color::GREEN.into(),
                                font_size: 40.0,
                                ..default()
                            },
                        )],
                        ..default()
                    },
                    ..default()
                },
                ButtonType::Quit,
            ));
        })
        .id();

    // Set up parent-child relationships
    commands
        .entity(main_menu_entity)
        .push_children(&[title_entity, buttons_entity]);
}

#[derive(Component)]
pub struct ScoreUi;

#[derive(Component)]
pub struct LivesUi;

#[derive(Component)]
pub struct LevelUi;

pub fn build_game_ui(mut commands: Commands, game_assets: Res<GameAssets>) {
    let player_icon = &game_assets.player_texture;
    // Load Ui images
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Top UI bar
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(3.0),
                        flex_direction: FlexDirection::Row, // Use row layout
                        justify_content: JustifyContent::SpaceBetween, // Distributes space between children
                        ..default()
                    },
                    background_color: TEXT_BACKGROUND.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Lives Count on the left
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(33.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Start,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    height: Val::Px(16.0),
                                    width: Val::Px(16.0),
                                    ..default()
                                },
                                image: UiImage::new(player_icon.clone()),
                                ..default()
                            });
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "x 1",
                                        TextStyle {
                                            font_size: TEXT_SIZE,
                                            color: TEXT_COLOR,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                },
                                LivesUi,
                            ));
                        });

                    // Score in the center
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(34.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "0",
                                        TextStyle {
                                            font_size: TEXT_SIZE,
                                            color: TEXT_COLOR,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                },
                                ScoreUi,
                            ));
                        });

                    // Level Count on the right
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(33.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::End,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "1",
                                        TextStyle {
                                            font_size: TEXT_SIZE,
                                            color: TEXT_COLOR,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                },
                                LevelUi,
                            ));
                        });
                });
        });
}

pub fn update_score_ui(mut score_query: Query<&mut Text, With<ScoreUi>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in score_query.iter_mut() {
            text.sections[0].value = format!("{:07}", score.0);
        }
    }
}

pub fn update_lives_ui(mut lives_query: Query<&mut Text, With<LivesUi>>, lives: Res<Lives>) {
    if lives.is_changed() {
        for mut text in lives_query.iter_mut() {
            text.sections[0].value = format!("x {}", lives.0);
        }
    }
}

pub fn update_level_ui(mut level_query: Query<&mut Text, With<LevelUi>>, level: Res<Level>) {
    if level.is_changed() {
        for mut text in level_query.iter_mut() {
            text.sections[0].value = format!("Level {}", level.0);
        }
    }
}

pub fn handle_button_navigation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected_button: ResMut<SelectedButton>,
    app_state: Res<State<AppState>>,
) {
    // Main Menu
    if *app_state.get() == AppState::MainMenu {
        if keyboard_input.just_pressed(UP) || keyboard_input.just_pressed(DOWN) {
            selected_button.0 = match selected_button.0 {
                ButtonType::Play => ButtonType::Quit,
                ButtonType::Quit => ButtonType::Play,
                _ => unreachable!("Tried to set button not in main menu!"),
            }
        }
    }

    // Game Over Screen
    if *app_state.get() == AppState::GameOver {
        if keyboard_input.just_pressed(UP) || keyboard_input.just_pressed(DOWN) {
            selected_button.0 = match selected_button.0 {
                ButtonType::Restart => ButtonType::MainMenu,
                ButtonType::MainMenu => ButtonType::Restart,
                _ => unreachable!("Tried to set button not in game over screen!"),
            };
        }
    }
}

pub fn set_default_button_selection(app_state: Res<State<AppState>>,
                                    mut selected_button: ResMut<SelectedButton>,) {
    // Set default button, this is to fix the bug that when going into game over,
    // the selected button from the main menu is leftover
    match *app_state.get() {
        AppState::MainMenu => selected_button.0 = ButtonType::Play,
        AppState::GameOver => selected_button.0 = ButtonType::Restart,
        _ => unreachable!("Tried to set the default button to an invalid state!"),
    }
}

pub fn update_button_colors(
    selected_button: Res<SelectedButton>,
    mut query: Query<(&ButtonType, &mut BackgroundColor)>,
) {
    for (button_type, mut background_color) in query.iter_mut() {
        if *button_type == selected_button.0 {
            *background_color = BUTTON_HOVER_COLOR.into();
        } else {
            *background_color = BUTTON_NORMAL_COLOR.into();
        }
    }
}

pub fn handle_button_actions(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    selected_button: Res<SelectedButton>,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(SHOOT_KEY) {
        match selected_button.0 {
            ButtonType::Play => {
                // Start the game
                next_app_state.set(AppState::InGame);
            }
            ButtonType::Quit => {
                // Quit the game
                app_exit_events.send(AppExit);
            }
            ButtonType::MainMenu => {
                next_app_state.set(AppState::MainMenu);
            }
            ButtonType::Restart => {
                next_app_state.set(AppState::InGame);
            }
        }
    }
}

#[derive(Component)]
pub struct GameOverUI;

pub fn spawn_game_over_ui(mut commands: Commands,  score: Res<Score>) {
    build_game_over_ui(&mut commands, &score);
}

pub fn despawn_game_over_ui(mut commands: Commands, game_over_ui_query: Query<Entity, With<GameOverUI>>) {
    if let Ok(game_over_ui_entity) = game_over_ui_query.get_single() {
        commands.entity(game_over_ui_entity).despawn_recursive();
    }
}

pub fn build_game_over_ui(commands: &mut Commands, score: &Res<Score>) {
    // Create the root node for the game over screen
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.95).into(), // Translucent background
            ..default()
        },
        GameOverUI))
        .with_children(|parent| {
            // Add a vertical layout container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "GAME OVER",
                                    TextStyle {
                                        font_size: 60.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                ..default()
                            });
                        });                    
                    
                    // Add some spacing between the texts
                    parent.spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(80.0),
                            ..default()
                        },
                        ..default()
                    });

                    // Add a container for final score text
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    format!("{:07}", score.0),
                                    TextStyle {
                                        font_size: 40.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                ..default()
                            });
                        });

                    // Define the button node
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(60.0),
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceEvenly,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Restart Button
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(200.0),
                                        height: Val::Px(80.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: BUTTON_NORMAL_COLOR.into(),
                                    ..default()
                                },
                                ButtonType::Restart,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        "Restart",
                                        TextStyle {
                                            font_size: 40.0,
                                            color: Color::GREEN,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                });
                            });

                            // Main Menu Button
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(200.0), 
                                        height: Val::Px(80.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: BUTTON_NORMAL_COLOR.into(),
                                    ..default()
                                },
                                ButtonType::MainMenu,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        "Main Menu",
                                        TextStyle {
                                            font_size: 40.0,
                                            color: Color::GREEN,
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                });
                            });
                        });
                });
        });
}
