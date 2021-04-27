// disable console opening on windows
// #![windows_subsystem = "windows"]

mod game;
mod loading;
mod menu;

use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};
use game::{config::CONFIG, GamePlugin};
use loading::{LoadingPlugin, SoundAssets};
use menu::MenuPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Game,
    GameOver,
    Pause,
    Menu,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Breakout".to_string(),
            width: CONFIG.play_area.width + 2.0 * CONFIG.wall_thickness,
            height: CONFIG.play_area.height + 2.0 * CONFIG.wall_thickness,
            resizable: false,
            vsync: false,
            ..Default::default()
        })
        .add_startup_system(load_cameras.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_state(GameState::Loading)
        .insert_resource(MusicChannel {
            music: AudioChannel::new("music".to_owned()),
        })
        .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(play_music.system()))
        .run();
}

fn load_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

struct MusicChannel {
    music: AudioChannel,
}

fn play_music(sounds: Res<SoundAssets>, audio: Res<Audio>, channels: Res<MusicChannel>) {
    audio.set_volume_in_channel(0.7, &channels.music);
    audio.play_looped_in_channel(sounds.music.clone(), &channels.music);
}
