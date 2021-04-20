use crate::{loading::FontAssets, GameState};
use bevy::{app::AppExit, prelude::*};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Menu).with_system(click_play_button.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup.system()));
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("world.assets#colorMaterial");
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

enum MenuButton {
    Play,
    Exit,
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    windows: Res<Windows>,
    button_materials: Res<ButtonMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let button_size = Size::new(Val::Px(160.0), Val::Px(50.0));
    let (top, left) = if let Some(window) = windows.get_primary() {
        (window.height() / 2.0, window.width() / 2.0 - 80.0)
    } else {
        (300.0, 100.0)
    };

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: button_size,
                // margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(top - 60.0),
                    left: Val::Px(left),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .insert(MenuButton::Play)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: font_assets.text_font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: button_size,
                // margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(top + 10.0),
                    left: Val::Px(left),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .insert(MenuButton::Exit)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Exit".to_string(),
                        style: TextStyle {
                            font: font_assets.text_font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

type ButtonInteraction<'a> = (
    &'a Interaction,
    &'a mut Handle<ColorMaterial>,
    &'a MenuButton,
);

fn click_play_button(
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<MenuButton>)>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut material, menu_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => match menu_button {
                MenuButton::Play => state.set(GameState::Game).expect("state: menu -> game"),
                MenuButton::Exit => app_exit_events.send(AppExit),
            },
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn cleanup(mut commands: Commands, buttons: Query<(Entity, &MenuButton)>) {
    for (entity, _button) in buttons.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
