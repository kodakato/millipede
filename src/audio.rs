use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Resource)]
pub struct AudioHandles{
    background_beat_handle: Handle<AudioSource>,
    millipede_audio_handle: Handle<AudioSource>,
    spider_audio_handle: Handle<AudioSource>,
    scorpion_audio_handle: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct BackgroundChannel;

#[derive(Resource)]
pub struct MillipedeChannel;

#[derive(Resource)]
pub struct SpiderChannel;

#[derive(Resource)]
pub struct ScorpionChannel;



pub fn prepare_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_beat_handle: Handle<AudioSource> = asset_server.load("sounds/background_beat.ogg");
    let millipede_audio_handle: Handle<AudioSource> = asset_server.load("sounds/millipede.ogg");
    let spider_audio_handle: Handle<AudioSource> = asset_server.load("sounds/spider.ogg");
    let scorpion_audio_handle: Handle<AudioSource> = asset_server.load("sounds/scorpion.ogg");
    
    commands.insert_resource(AudioHandles {
        background_beat_handle,
        millipede_audio_handle,
        spider_audio_handle,
        scorpion_audio_handle,
    });    
}

pub fn start_background_audio(
    audio: Res<Audio>,
    audio_handles: Res<AudioHandles>,
    background_channel: Res<AudioChannel<BackgroundChannel>>,
    millipede_channel: Res<AudioChannel<MillipedeChannel>>,
    spider_channel: Res<AudioChannel<SpiderChannel>>,
    scorpion_channel: Res<AudioChannel<ScorpionChannel>>,
) {
    background_channel.play(audio_handles.background_beat_handle.clone()).looped();
    millipede_channel.play(audio_handles.millipede_audio_handle.clone()).looped();
    spider_channel.play(audio_handles.spider_audio_handle.clone()).looped();
    scorpion_channel.play(audio_handles.scorpion_audio_handle.clone()).looped();
}

pub fn set_initial_volumes(
    millipede_channel: Res<AudioChannel<MillipedeChannel>>,
    spider_channel: Res<AudioChannel<SpiderChannel>>,
    scorpion_channel: Res<AudioChannel<ScorpionChannel>>,
) {
    millipede_channel.set_volume(1.0);
    spider_channel.set_volume(1.0);
    scorpion_channel.set_volume(1.0);
}
