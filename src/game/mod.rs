use crate::{loading::SoundAssets, GameState};
use balls::Ball;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};

use self::{
    balls::BallPlugin, bricks::BrickPlugin, gameover::GameOverPlugin, paddle::PaddlePlugin,
    pause::PausePlugin, walls::WallPlugin,
};

mod balls;
mod bricks;
pub mod config;
mod gameover;
mod paddle;
mod pause;
pub mod walls;

pub struct Disposable;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(hide_mouse.system()));

        app.add_system(handle_keyboard_esc.system());

        app.add_system_set(
            SystemSet::on_exit(GameState::Game).with_system(cleanup.system()), // .with_system(stop_music.system()),
        );

        app.add_plugin(WallPlugin);
        app.add_plugin(BrickPlugin);
        app.add_plugin(BallPlugin);
        app.add_plugin(PaddlePlugin);
        app.add_plugin(PausePlugin);
        app.add_plugin(GameOverPlugin);
    }
}

fn hide_mouse(mut windows: ResMut<Windows>, mut mouse_button_input: ResMut<Input<MouseButton>>) {
    mouse_button_input.reset(MouseButton::Left);
    if let Some(window) = windows.get_primary_mut() {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
}

fn handle_keyboard_esc(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    match state.current() {
        GameState::Game | GameState::GameOver | GameState::Pause => {
            if keyboard_input.just_pressed(KeyCode::Escape) {
                keyboard_input.reset(KeyCode::Escape);
                state.replace(GameState::Menu).expect("state: game -> menu");
            }
        }
        _ => {}
    }
}

fn cleanup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    entities: Query<(Entity, &Disposable)>,
) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }

    for (entity, _disposable) in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
