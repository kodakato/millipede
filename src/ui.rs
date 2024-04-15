use crate::{constants::*, game::{Score, player::Lives, level::Level, assets::*}};
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

// Main Menu
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "Millipede!",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::GREEN,
                        ..default()
                    },
                ),
                ..default()
            },));
        })
        .id();
    main_menu_entity
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

pub fn update_score_ui(
    mut score_query: Query<&mut Text, With<ScoreUi>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        for mut text in score_query.iter_mut() {
            text.sections[0].value = format!("{:07}", score.0);
        }
    }
}

pub fn update_lives_ui(
    mut lives_query: Query<&mut Text, With<LivesUi>>,
    lives: Res<Lives>,
) {
    if lives.is_changed() {
        for mut text in lives_query.iter_mut() {
            text.sections[0].value = format!("x {}", lives.0);
        }
    }
}

pub fn update_level_ui(
    mut level_query: Query<&mut Text, With<LevelUi>>,
    level: Res<Level>,
) {
    if level.is_changed() {
        for mut text in level_query.iter_mut() {
            text.sections[0].value = format!("Level {}", level.0);
        }
    }
}
