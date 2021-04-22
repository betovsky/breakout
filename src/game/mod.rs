use crate::{loading::*, GameState};
use balls::Ball;
use bevy::prelude::*;

use self::{balls::BallPlugin, bricks::BrickPlugin, paddle::PaddlePlugin, walls::WallPlugin};

mod balls;
mod bricks;
pub mod config;
mod paddle;
pub mod walls;

pub struct Disposable;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PauseMaterials>();

        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(hide_mouse.system()));

        app.add_system_set(
            SystemSet::on_update(GameState::Game).with_system(handle_keyboard.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Pause).with_system(handle_keyboard.system()),
        );

        app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(cleanup.system()));

        app.add_plugin(WallPlugin);
        app.add_plugin(BrickPlugin);
        app.add_plugin(BallPlugin);
        app.add_plugin(PaddlePlugin);
    }
}

struct PauseMaterials {
    background: Handle<ColorMaterial>,
}

impl FromWorld for PauseMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("world.assets#colorMaterial");
        PauseMaterials {
            background: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.4).into()),
        }
    }
}

fn hide_mouse(mut windows: ResMut<Windows>, mut mouse_button_input: ResMut<Input<MouseButton>>) {
    mouse_button_input.reset(MouseButton::Left);
    if let Some(window) = windows.get_primary_mut() {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
}

fn handle_keyboard(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    font_assets: Res<FontAssets>,
    pause_materials: Res<PauseMaterials>,
    text_entity: Query<(Entity, &Text)>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        keyboard_input.reset(KeyCode::Escape);
        state.replace(GameState::Menu).expect("state: game -> menu");
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        keyboard_input.reset(KeyCode::Space);
        match state.current() {
            GameState::Pause => {
                for (entity, _text) in text_entity.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                state.pop()
            }
            _ => {
                commands
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Auto),
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Percent(15.0),
                                left: Val::Percent(10.0),
                                right: Val::Percent(10.0),
                                ..Default::default()
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::FlexEnd,
                            ..Default::default()
                        },
                        material: pause_materials.background.clone(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                ..Default::default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    value: "PAUSED".to_string(),
                                    style: TextStyle {
                                        font: font_assets.text_font.clone(),
                                        font_size: 72.0,
                                        color: Color::ORANGE,
                                    },
                                }],
                                alignment: TextAlignment {
                                    horizontal: HorizontalAlign::Center,
                                    vertical: VerticalAlign::Center,
                                },
                            },
                            ..Default::default()
                        });
                    })
                    .insert(Disposable);
                state.push(GameState::Pause)
            }
        }
        .expect("state: pause");
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
