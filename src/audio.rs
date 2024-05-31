use crate::{constants::*, AppState, GameState};
use bevy::{prelude::*, utils::Duration, window::PrimaryWindow};
use bevy_kira_audio::prelude::*;

use crate::game::{millipede::Segment, scorpion::Scorpion, spider::Spider};

#[derive(Resource)]
pub struct Instances {
    background_beat: (Handle<AudioInstance>, f64),
    millipede: (Handle<AudioInstance>, f64),
    spider: (Handle<AudioInstance>, f64),
    scorpion: (Handle<AudioInstance>, f64),
    highhat: (Handle<AudioInstance>, f64),
}

pub fn prepare_audio(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    let background_beat_handle = audio
        .play(asset_server.load("sounds/background_beat.ogg"))
        .looped()
        .handle();
    let millipede_handle = audio
        .play(asset_server.load("sounds/millipede.ogg"))
        .looped()
        .handle();
    let spider_handle = audio
        .play(asset_server.load("sounds/spider.ogg"))
        .looped()
        .handle();
    let scorpion_handle = audio
        .play(asset_server.load("sounds/scorpion.ogg"))
        .looped()
        .handle();
    let highhat_handle = audio
        .play(asset_server.load("sounds/highhat.ogg"))
        .looped()
        .handle();

    let background_beat = (background_beat_handle.clone(), 1.0);
    let millipede = (millipede_handle.clone(), 1.0);
    let spider = (spider_handle.clone(), 1.0);
    let scorpion = (scorpion_handle.clone(), 1.0);
    let highhat = (highhat_handle.clone(), 1.0);

    commands.insert_resource(Instances {
        background_beat,
        millipede,
        spider,
        scorpion,
        highhat,
    });
}

pub fn initialize_volume(
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut instances: ResMut<Instances>,
) {
    let millipede_handle = &instances.millipede.0;
    // Millipede
    if let Some(instance) = audio_instances.get_mut(millipede_handle) {
        instance.set_volume(0.0, AudioTween::default());
        instances.millipede.1 = 0.0;
    }

    let spider_handle = &instances.spider.0;
    // Spider
    if let Some(instance) = audio_instances.get_mut(spider_handle) {
        instance.set_volume(0.0, AudioTween::default());
        instances.spider.1 = 0.0;
    }

    let scorpion_handle = &instances.scorpion.0;
    // Scorpion
    if let Some(instance) = audio_instances.get_mut(scorpion_handle) {
        instance.set_volume(0.0, AudioTween::default());
        instances.spider.1 = 0.0;
    }
    
    let highhat_handle = &instances.highhat.0;
    if let Some(instance) = audio_instances.get_mut(highhat_handle) {
        instance.set_volume(0.0, AudioTween::default());
        instances.spider.1 = 0.0;
    }
   
}

pub fn set_volume(
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut instances: ResMut<Instances>,
    millipede_query: Query<(), With<Segment>>,
    spider_query: Query<(), With<Spider>>,
    scorpion_query: Query<(), With<Scorpion>>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
) {
    let millipede_handle = &instances.millipede.0;
    // Millipede
    if !millipede_query.is_empty() && app_state.get() == &AppState::InGame{
        if instances.millipede.1 != MILLIPEDE_VOLUME {
            if let Some(instance) = audio_instances.get_mut(millipede_handle) {
                instance.set_volume(
                    MILLIPEDE_VOLUME,
                    AudioTween::linear(Duration::from_secs_f32(0.5)),
                );
                instances.millipede.1 = MILLIPEDE_VOLUME;
            }
        }
    } else {
        if instances.millipede.1 != 0.0 {
            if let Some(instance) = audio_instances.get_mut(millipede_handle) {
                instance.set_volume(0.0, AudioTween::default());
                instances.millipede.1 = 0.0;
            }
        }
    }

    let spider_handle = &instances.spider.0;
    // Spider
    if !spider_query.is_empty() && app_state.get() == &AppState::InGame{
        if instances.spider.1 != SPIDER_VOLUME {
            if let Some(instance) = audio_instances.get_mut(spider_handle) {
                instance.set_volume(
                    SPIDER_VOLUME,
                    AudioTween::linear(Duration::from_secs_f32(0.5)),
                );
                instances.spider.1 = SPIDER_VOLUME;
            }
        }
    } else {
        if instances.spider.1 != 0.0 {
            if let Some(instance) = audio_instances.get_mut(spider_handle) {
                instance.set_volume(0.0, AudioTween::default());
                instances.spider.1 = 0.0
            }
        }
    }

    let scorpion_handle = &instances.scorpion.0;
    // Scorpion
    if !scorpion_query.is_empty() {
        if instances.scorpion.1 != SCORPION_VOLUME  && app_state.get() == &AppState::InGame{
            if let Some(instance) = audio_instances.get_mut(scorpion_handle) {
                instance.set_volume(SCORPION_VOLUME, AudioTween::default());
                instances.scorpion.1 = SCORPION_VOLUME;
            }
        }
    } else {
        if instances.scorpion.1 != 0.0 {
            if let Some(instance) = audio_instances.get_mut(scorpion_handle) {
                instance.set_volume(0.0, AudioTween::default());
                instances.scorpion.1 = 0.0;
            }
        }
    }
    let highhat_handle = &instances.highhat.0;
    match *app_state.get() {
        AppState::InGame => {
            if instances.highhat.1 != BACKGROUND_VOLUME  && game_state.get() == &GameState::Running {
                if let Some(instance) = audio_instances.get_mut(highhat_handle) {
                    instance.set_volume(BACKGROUND_VOLUME, AudioTween::default());
                    instances.highhat.1 = BACKGROUND_VOLUME;
                }
            }
        }
        _ => {
            if let Some(instance) = audio_instances.get_mut(highhat_handle) {
                instance.set_volume(0.0, AudioTween::default());
                instances.highhat.1 = 0.0;
            }
        }
    }
}

pub fn sync_audio(
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    instances: ResMut<Instances>,
) {
    let background_beat_handle = &instances.background_beat.0;
    if let Some(background_instance) = audio_instances.get_mut(background_beat_handle) {
        match background_instance.state() {
            PlaybackState::Playing { position } => {
                if position <= 0.2 {
                    // Set the position of each track to 0
                    let spider_handle = &instances.spider.0;
                    if let Some(spider_instance) = audio_instances.get_mut(spider_handle) {
                        spider_instance.seek_to(position);
                    }

                    let millipede_handle = &instances.millipede.0;
                    if let Some(millipede_instance) = audio_instances.get_mut(millipede_handle) {
                        millipede_instance.seek_to(position);
                    }

                    let scorpion_handle = &instances.scorpion.0;
                    if let Some(scorpion_instance) = audio_instances.get_mut(scorpion_handle) {
                        scorpion_instance.seek_to(position);
                    }

                    let highhat_handle = &instances.highhat.0;
                    if let Some(highat_instance) = audio_instances.get_mut(highhat_handle) {
                        highat_instance.seek_to(position);
                    }
                }
            }
            _ => return,
        }
    }
}
