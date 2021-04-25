use super::{config::CONFIG, Ball, Disposable};
use crate::{loading::MaterialsAssets, GameState};
use bevy::{
    input::mouse::MouseMotion,
    math::Vec2,
    prelude::*,
    sprite::{collide_aabb::collide, collide_aabb::Collision, Sprite},
};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_board.system()));
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(start_ball.system()));

        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .label("movement")
                .with_system(paddle_movement.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .after("movement")
                .with_system(ball_paddle_collision.system()),
        );
    }
}

pub struct Paddle;

fn setup_board(mut commands: Commands, materials: Res<MaterialsAssets>) {
    // paddle
    let base_line = -(CONFIG.play_area.height / 2.0) + 50.0;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.paddle.clone(),
            sprite: Sprite::new(Vec2::new(
                CONFIG.paddle_starting_size.width,
                CONFIG.paddle_starting_size.height,
            )),
            transform: Transform::from_translation(Vec3::new(0., base_line, 1.)),
            ..Default::default()
        })
        .insert(Paddle)
        .insert(Disposable);
}

fn paddle_movement(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    if let Ok((_paddle, mut transform)) = query.single_mut() {
        let delta: f32 = mouse_motion_events.iter().map(|e| e.delta.x).sum();
        transform.translation.x += delta;

        let limit = (CONFIG.play_area.width / 2.0) - (CONFIG.paddle_starting_size.width / 2.0);
        transform.translation.x = transform.translation.x.clamp(-limit, limit);
    }
}

fn ball_paddle_collision(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    paddle_query: Query<(&Paddle, &Transform, &Sprite)>,
) {
    if let Ok((_paddle, paddle_transform, paddle_sprite)) = paddle_query.single() {
        for (mut ball, ball_transform, ball_sprite) in ball_query.iter_mut() {
            let ball_size = ball_sprite.size;
            let ball_position = ball_transform.translation;

            let paddle_size = paddle_sprite.size;
            let paddle_position = paddle_transform.translation;

            let collision = collide(ball_position, ball_size, paddle_position, paddle_size);

            if let Some(collision) = collision {
                match collision {
                    Collision::Top => {
                        // adjust velocity on x-axis depending of where it hit on the paddle
                        let mut velocity = ball.velocity;
                        velocity.y = velocity.y.abs();
                        velocity.x += 2.0 * (ball_position.x - paddle_position.x);

                        // for each time it hits the paddle, increase the ball's speed
                        ball.speed = (ball.speed + 20.0).min(1600.0);
                        println!("Speed: {:?}", ball.speed);
                        ball.velocity = ball.speed * velocity.normalize();
                    }
                    _ => ball.velocity.x *= -1.0,
                }
            }
        }
    }
}

fn start_ball(
    mouse_button_input: Res<Input<MouseButton>>,
    mut ball_query: Query<&mut Ball>,
    paddle_query: Query<(&Paddle, &Transform)>,
) {
    if let Ok(mut ball) = ball_query.single_mut() {
        if mouse_button_input.just_pressed(MouseButton::Left) && ball.velocity == Vec3::default() {
            let paddle_x = paddle_query
                .single()
                .map_or(0.0, |(_paddle, transform)| transform.translation.x);

            let direction =
                Vec3::new(-paddle_x, CONFIG.paddle_starting_size.width, 0.0).normalize();
            ball.velocity = CONFIG.ball_starting_speed * direction;
            ball.speed = CONFIG.ball_starting_speed;
        }
    }
}
