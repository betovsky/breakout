use bevy::prelude::*;

use crate::{loading::FontAssets, GameState};

use super::Disposable;

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PauseMaterials>();

        app.add_system_set(
            SystemSet::on_update(GameState::Game).with_system(handle_keyboard.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Pause).with_system(handle_keyboard.system()),
        );
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

fn handle_keyboard(
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    font_assets: Res<FontAssets>,
    pause_materials: Res<PauseMaterials>,
    text_entity: Query<(Entity, &Text)>,
) {
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
