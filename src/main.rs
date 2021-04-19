// disable console opening on windows
// #![windows_subsystem = "windows"]

mod game;
mod loading;

use bevy::render::pass::ClearColor;
use bevy::{input::system::exit_on_esc_system, prelude::*};
use game::{GamePlugin, PLAYAREA_HEIGHT, PLAYAREA_WIDTH, WALL_THICKNESS};
use loading::LoadingPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Game,
    Pause,
    Menu,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Breakout".to_string(),
            width: PLAYAREA_WIDTH + 2.0 * WALL_THICKNESS,
            height: PLAYAREA_HEIGHT + 2.0 * WALL_THICKNESS,
            resizable: false,
            cursor_locked: true,
            cursor_visible: false,
            vsync: false,
            ..Default::default()
        })
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugin(GamePlugin)
        .add_state(GameState::Loading)
        .run();
}
