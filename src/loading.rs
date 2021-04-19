use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_state.system()));
    }
}

struct AssetPaths {
    zen_dots: &'static str,
    ball: &'static str,
    paddle: &'static str,
    bricks: &'static str,
}

const PATHS: AssetPaths = AssetPaths {
    zen_dots: "fonts/ZenDots-Regular.ttf",
    ball: "images/silver-ball.png",
    paddle: "images/wood-texture.png",
    bricks: "images/bricks.png",
};

pub struct MaterialsAssets {
    pub paddle: Handle<ColorMaterial>,
    pub wall: Handle<ColorMaterial>,
    pub ball: Handle<ColorMaterial>,
}

pub struct BrickAssets {
    pub textures: Handle<TextureAtlas>,
}
pub struct FontAssets {
    pub text_font: Handle<Font>,
}

struct LoadingState {
    images: Vec<HandleUntyped>,
    fonts: Vec<HandleUntyped>,
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("start_loading");
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.zen_dots));

    let mut images: Vec<HandleUntyped> = vec![];
    images.push(asset_server.load_untyped(PATHS.ball));
    images.push(asset_server.load_untyped(PATHS.paddle));
    images.push(asset_server.load_untyped(PATHS.bricks));

    commands.insert_resource(LoadingState { images, fonts });
}

fn check_state(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    loading_state: Res<LoadingState>,
) {
    if !has_loaded(&asset_server, &loading_state.fonts) {
        return;
    }

    if !has_loaded(&asset_server, &loading_state.images) {
        return;
    }

    commands.insert_resource(FontAssets {
        text_font: asset_server.get_handle(PATHS.zen_dots),
    });

    commands.insert_resource(MaterialsAssets {
        ball: materials.add(asset_server.get_handle(PATHS.ball).into()),
        paddle: materials.add(asset_server.get_handle(PATHS.paddle).into()),
        wall: materials.add(Color::GRAY.into()),
    });

    commands.insert_resource(BrickAssets {
        textures: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.get_handle(PATHS.bricks),
            Vec2::new(200.0, 70.0),
            3,
            1,
        )),
    });

    state.set(GameState::Game).expect("state: menu");
    println!("finish loading");
}

fn has_loaded(asset_server: &Res<AssetServer>, assets: &Vec<HandleUntyped>) -> bool {
    LoadState::Loaded == asset_server.get_group_load_state(assets.iter().map(|handle| handle.id))
}
