use bevy::prelude::*;

use crate::{loading::FontAssets, GameState};

use super::{balls::Ball, Disposable};

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GameOverMaterials>();

        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(game_over.system()));
    }
}

struct GameOverMaterials {
    background: Handle<ColorMaterial>,
}

impl FromWorld for GameOverMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("world.assets#colorMaterial");
        GameOverMaterials {
            background: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.4).into()),
        }
    }
}

fn game_over(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    font_assets: Res<FontAssets>,
    materials: Res<GameOverMaterials>,
    balls_query: Query<&Ball>,
) {
    if balls_query.iter().next().is_none() {
        // not more balls left... gameover!
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
                material: materials.background.clone(),
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
                            value: "GAME OVER".to_string(),
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
        state
            .push(GameState::GameOver)
            .expect("state: game -> gameover");
    }
}
