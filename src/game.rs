use crate::{loading::*, GameState};
use bevy::{input::mouse::MouseMotion, sprite::collide_aabb::collide};
use bevy::{prelude::*, sprite::collide_aabb::Collision};
use rand::{distributions::Uniform, prelude::Distribution};

pub const PLAYAREA_WIDTH: f32 = 600.0;
pub const PLAYAREA_HEIGHT: f32 = 800.0;
pub const WALL_THICKNESS: f32 = 10.0;
const BRICK_WIDTH: f32 = 40.0;
const BRICK_HEIGHT: f32 = 20.0;
const BRICK_ROWS: i32 = 10;
const PADDLE_WIDTH: f32 = 140.0;
const PADDLE_HEIGHT: f32 = 20.0;
const BALL_STARTING_SPEED: f32 = 300.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(setup_board.system())
                .with_system(hide_mouse.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .label("movement")
                .with_system(paddle_movement.system())
                .with_system(ball_movement.system()),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .after("movement")
                .with_system(ball_wall_collision.system())
                .with_system(ball_paddle_collision.system())
                .with_system(brick_collision.system()),
        );

        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(start_ball.system()));
    }
}

struct Paddle;
struct Wall;
struct Brick {
    life: u32,
}

struct Ball {
    velocity: Vec3,
    speed: f32,
}

fn hide_mouse(mut windows: ResMut<Windows>, mut mouse_button_input: ResMut<Input<MouseButton>>) {
    mouse_button_input.reset(MouseButton::Left);
    if let Some(window) = windows.get_primary_mut() {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
}

fn setup_board(
    mut commands: Commands,
    materials: Res<MaterialsAssets>,
    brick_assets: Res<BrickAssets>,
) {
    println!("setup game");
    // walls
    setup_walls(&mut commands, &materials);

    // paddle
    let base_line = -(PLAYAREA_HEIGHT / 2.0) + 50.0;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.paddle.clone(),
            sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            transform: Transform::from_translation(Vec3::new(0., base_line, 1.)),
            ..Default::default()
        })
        .insert(Paddle);

    // ball
    let starting_height = base_line + (PADDLE_HEIGHT / 2.0) + 7.0;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.ball.clone(),
            sprite: Sprite::new(Vec2::new(14., 14.)),
            transform: Transform::from_translation(Vec3::new(0.0, starting_height, 1.0)),
            ..Default::default()
        })
        .insert(Ball {
            velocity: Vec3::default(),
            speed: 0.0,
        });

    // bricks
    let between = Uniform::from(0..3u32);
    let mut rng = rand::thread_rng();
    let bricks_per_row = (PLAYAREA_WIDTH / BRICK_WIDTH) as i32;
    for row_index in 0..BRICK_ROWS {
        let ri = row_index as f32;
        let starting_height = PLAYAREA_HEIGHT / 2.0 - BRICK_HEIGHT / 2.0 - ri * BRICK_HEIGHT;
        for column_index in 0..bricks_per_row {
            let ci = column_index as f32;
            let brick_life = between.sample(&mut rng);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: brick_assets.textures.clone(),
                    sprite: TextureAtlasSprite {
                        index: brick_life,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            ci * BRICK_WIDTH - PLAYAREA_WIDTH / 2.0 + BRICK_WIDTH / 2.0,
                            starting_height,
                            1.0,
                        ),
                        scale: Vec3::new(BRICK_WIDTH / 200.0, BRICK_HEIGHT / 70.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Brick { life: brick_life });
        }
    }

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_walls(commands: &mut Commands, materials: &Res<MaterialsAssets>) {
    // Add walls
    // left
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.wall.clone(),
            transform: Transform::from_xyz(-PLAYAREA_WIDTH / 2.0 - WALL_THICKNESS / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(
                WALL_THICKNESS,
                PLAYAREA_HEIGHT + 2.0 * WALL_THICKNESS,
            )),
            ..Default::default()
        })
        .insert(Wall);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.wall.clone(),
            transform: Transform::from_xyz(PLAYAREA_WIDTH / 2.0 + WALL_THICKNESS / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(
                WALL_THICKNESS,
                PLAYAREA_HEIGHT + 2.0 * WALL_THICKNESS,
            )),
            ..Default::default()
        })
        .insert(Wall);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.wall.clone(),
            transform: Transform::from_xyz(0.0, PLAYAREA_HEIGHT / 2.0 + WALL_THICKNESS / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(
                PLAYAREA_WIDTH + 2.0 * WALL_THICKNESS,
                WALL_THICKNESS,
            )),
            ..Default::default()
        })
        .insert(Wall);
}

fn paddle_movement(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    if let Ok((_paddle, mut transform)) = query.single_mut() {
        let delta: f32 = mouse_motion_events.iter().map(|e| e.delta.x).sum();
        transform.translation.x += delta;

        let limit = (PLAYAREA_WIDTH / 2.0) - (PADDLE_WIDTH / 2.0);
        transform.translation.x = transform.translation.x.clamp(-limit, limit);
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

            let direction = Vec3::new(-paddle_x, PADDLE_WIDTH, 0.0).normalize();
            ball.velocity = BALL_STARTING_SPEED * direction;
            ball.speed = BALL_STARTING_SPEED;
        }
    }
}

fn ball_movement(timer: Res<Time>, mut ball_query: Query<(&Ball, &mut Transform)>) {
    if let Ok((ball, mut transform)) = ball_query.single_mut() {
        transform.translation += ball.velocity * timer.delta_seconds();
    }
}

fn ball_wall_collision(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    wall_query: Query<(&Wall, &Transform, &Sprite)>,
) {
    if let Ok((mut ball, ball_transform, ball_sprite)) = ball_query.single_mut() {
        let ball_size = ball_sprite.size;
        let ball_position = ball_transform.translation;

        for (_wall, transform, sprite) in wall_query.iter() {
            let collision = collide(ball_position, ball_size, transform.translation, sprite.size);

            if let Some(collision) = collision {
                match collision {
                    Collision::Left | Collision::Right => {
                        ball.velocity.x = ball.velocity.x.copysign(-ball_position.x)
                    }
                    Collision::Top | Collision::Bottom => ball.velocity.y = -ball.velocity.y.abs(),
                }
            }
        }
    }
}

fn ball_paddle_collision(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    paddle_query: Query<(&Paddle, &Transform, &Sprite)>,
) {
    if let Ok((mut ball, ball_transform, ball_sprite)) = ball_query.single_mut() {
        if let Ok((_paddle, paddle_transform, paddle_sprite)) = paddle_query.single() {
            let ball_size = ball_sprite.size;
            let ball_position = ball_transform.translation;

            let paddle_size = paddle_sprite.size;
            let paddle_position = paddle_transform.translation;

            let collision = collide(ball_position, ball_size, paddle_position, paddle_size);

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => ball.velocity.x *= -1.0,
                    Collision::Right => ball.velocity.x *= -1.0,
                    Collision::Bottom => unreachable!(), // maybe put some end case scenario??
                    Collision::Top => {
                        // adjust velocity on x-axis depending of where it hit on the paddle
                        let mut velocity = ball.velocity;
                        velocity.y = velocity.y.abs();
                        velocity.x += 2.0 * (ball_position.x - paddle_position.x);

                        // for each time it hits the paddle, increase the ball's speed
                        ball.speed = (ball.speed + 20.0).min(1000.0);
                        println!("Speed: {:?}", ball.speed);
                        ball.velocity = ball.speed * velocity.normalize();
                    }
                }
            }
        }
    }
}

fn brick_collision(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    mut brick_query: Query<(Entity, &mut Brick, &Transform, &mut TextureAtlasSprite)>,
    mut commands: Commands,
) {
    if let Ok((mut ball, ball_transform, ball_sprite)) = ball_query.single_mut() {
        let brick_size = Vec2::new(BRICK_WIDTH, BRICK_HEIGHT);
        for (entity, mut brick, transform, mut sprite) in brick_query.iter_mut() {
            let ball_size = ball_sprite.size;
            let ball_position = ball_transform.translation;

            let brick_position = transform.translation;

            let collision = collide(ball_position, ball_size, brick_position, brick_size);

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => ball.velocity.x = -ball.velocity.x.abs(),
                    Collision::Right => ball.velocity.x = ball.velocity.x.abs(),
                    Collision::Top => ball.velocity.y = ball.velocity.y.abs(),
                    Collision::Bottom => ball.velocity.y = -ball.velocity.y.abs(),
                };

                if brick.life > 0 {
                    brick.life -= 1;
                    sprite.index = brick.life;
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
