use bevy::prelude::*;
use crate::{Score, constants::*};

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
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
                style: Style{
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            MainMenu,
        )
    ).with_children(|parent| {
        parent.spawn(
            (
                TextBundle {
                    text: Text::from_section("Millipede!",
                                             TextStyle {
                                                 font_size: 60.0,
                                                 color: Color::GREEN,
                                                ..default()
                                             }),
                    ..default()
                },
            )
        );

    }
    )
    .id();
    main_menu_entity
}

#[derive(Component)]
pub struct ScoreUi;


pub fn build_game_ui(mut commands: Commands) {
    commands.spawn(
        (
            NodeBundle{
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }
        )
    ).with_children( |parent| {// Top UI bar
        parent.spawn(
            NodeBundle{
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(5.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }
        ).with_children(|parent| {
            parent.spawn(
                (
                    TextBundle {
                        text: Text::from_section("0",
                                                 TextStyle {
                                                     font_size: 20.0,
                                                     color: TEXT_COLOR,
                                                    ..default()
                                                 }).with_justify(JustifyText::Center),
                        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, TEXT_TRANSPARENCY)),
                        ..default()
                    },
                    ScoreUi,
                )
            );

        });

    }
    );
}

pub fn update_game_ui(mut commands: Commands, mut score_query: Query<&mut Text, With<ScoreUi>>, score: Res<Score>) {
    if !score.is_changed() {
        return
    }

    for mut text in score_query.iter_mut() {
            text.sections[0].value = format!("{:06}", score.0);
    }
}
