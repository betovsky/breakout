use super::{config::CONFIG, Disposable};
use crate::{loading::MaterialsAssets, GameState};
use bevy::{math::Vec3, prelude::*};

pub struct Ball {
    pub velocity: Vec3,
    pub speed: f32,
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_board.system()));

        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .label("movement")
                .with_system(ball_movement.system()),
        );
    }
}

fn setup_board(mut commands: Commands, materials: Res<MaterialsAssets>) {
    println!("setup game");
    // ball
    let base_line = -(CONFIG.play_area.height / 2.0) + 50.0;
    let starting_height = base_line + (CONFIG.paddle_starting_size.height / 2.0) + 7.0;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.ball.clone(),
            sprite: Sprite::new(Vec2::new(14., 14.)),
            transform: Transform::from_translation(Vec3::new(0.0, starting_height, 1.0)),
            ..Default::default()
        })
        .insert(Disposable)
        .insert(Ball {
            velocity: Vec3::default(),
            speed: 0.0,
        });
}

fn ball_movement(timer: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    if let Ok((ball, mut transform)) = ball_query.single_mut() {
        transform.translation += ball.velocity * timer.delta_seconds();
    }
}
