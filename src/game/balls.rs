use super::{config::CONFIG, Disposable};
use crate::{loading::MaterialsAssets, GameState};
use bevy::{math::Vec3, prelude::*};

#[derive(Default)]
pub struct Ball {
    pub velocity: Vec3,
    pub speed: f32,
}

impl Ball {
    pub fn with_default_speed(direction: Vec3) -> Self {
        Ball {
            velocity: direction.normalize() * CONFIG.ball_starting_speed,
            speed: CONFIG.ball_starting_speed,
        }
    }

    pub fn new(velocity: Vec3) -> Self {
        Ball {
            velocity: velocity,
            speed: velocity.length(),
        }
    }
}

impl Ball {
    pub fn spawn(
        commands: &mut Commands,
        materials: &Res<MaterialsAssets>,
        position: Vec2,
        ball: Ball,
    ) {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.ball.clone(),
                sprite: Sprite::new(Vec2::new(14., 14.)),
                transform: Transform::from_translation((position, 1.0).into()), // put on position with z: 1.0
                ..Default::default()
            })
            .insert(Disposable)
            .insert(ball);
    }
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
    Ball::spawn(
        &mut commands,
        &materials,
        Vec2::new(0.0, starting_height),
        Ball::default(),
    );
}

fn ball_movement(
    timer: Res<Time>,
    mut commands: Commands,
    mut balls_query: Query<(Entity, &Ball, &mut Transform)>,
) {
    let limit = -CONFIG.play_area.height / 2.0;
    for (entity, ball, mut transform) in balls_query.iter_mut() {
        transform.translation += ball.velocity * timer.delta_seconds();
        if transform.translation.y < limit {
            println!("Despawn ball");
            commands.entity(entity).despawn();
        }
    }
}
