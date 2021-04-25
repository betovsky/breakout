// disable console opening on windows
// #![windows_subsystem = "windows"]

mod game;
mod loading;
mod menu;

use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use game::{config::CONFIG, GamePlugin};
use loading::LoadingPlugin;
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
        .add_startup_system(load_camera2d.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_state(GameState::Loading)
        .run();
}

fn load_camera2d(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
