use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseMotion,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use bevy::{input::system::exit_on_esc_system, prelude::*};
use rand::{distributions::Uniform, prelude::Distribution};

const PLAYAREA_WIDTH: f32 = 600.0;
const PLAYAREA_HEIGHT: f32 = 800.0;
const BRICK_WIDTH: f32 = 40.0;
const BRICK_HEIGHT: f32 = 20.0;
const BRICK_ROWS: i32 = 10;
const WALL_THICKNESS: f32 = 10.0;
const PADDLE_WIDTH: f32 = 140.0;
const PADDLE_HEIGHT: f32 = 20.0;
const BALL_STARTING_SPEED: f32 = 300.0;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Breakout".to_string(),
            width: PLAYAREA_WIDTH + 2.0 * WALL_THICKNESS,
            height: PLAYAREA_HEIGHT + 2.0 * WALL_THICKNESS,
            resizable: false,
            cursor_locked: true,
            cursor_visible: false,
            vsync: false,
            ..Default::default()
        })
        .add_startup_system(setup_resources.system())
        .add_startup_stage("game_setup", SystemStage::single(setup.system()))
        .add_system(paddle_movement.system().label("movement"))
        .add_system(start_ball.system().label("movement"))
        .add_system(ball_movement.system().label("movement"))
        .add_system(ball_wall_collision.system().after("movement"))
        .add_system(ball_paddle_collision.system().after("movement"))
        .add_system(brick_collision.system().after("movement"))
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

struct MyMaterials {
    // bricks: TextureAtlas, //Vec<Handle<ColorMaterial>>,
    paddle: Handle<ColorMaterial>,
    wall: Handle<ColorMaterial>,
    ball: Handle<ColorMaterial>,
}

impl MyMaterials {
    fn init(asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        // let brick_texture_files = [
        //     "images/brickwall.png",
        //     "images/brickwall2.png",
        //     "images/cinder-block.png",
        // ];

        MyMaterials {
            // bricks: brick_texture_files
            //     .iter()
            //     .map(|&texture_file| {
            //         let handle = asset_server.load(texture_file);
            //         materials.add(handle.into())
            //     })
            //     .collect(),
            paddle: materials.add(asset_server.load("images/wood-texture.png").into()),
            wall: materials.add(Color::GRAY.into()),
            ball: materials.add(asset_server.load("images/silver-ball.png").into()),
        }
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

fn setup_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(MyMaterials::init(asset_server, materials));
    commands.insert_resource(ClearColor(Color::BLACK));
}

fn setup(
    mut commands: Commands,
    materials: Res<MyMaterials>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
    let texture_handle = asset_server.load("images/bricks.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 70.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

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
                    texture_atlas: texture_atlas_handle.clone(),
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

    // commands.insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)));
}

fn setup_walls(commands: &mut Commands, materials: &Res<MyMaterials>) {
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
        transform.translation.x = transform.translation.x.clamp(-limit, limit); //  .min(limit).max(-limit);
    }
}

fn start_ball(
    mouse_button_input: Res<Input<MouseButton>>,
    mut ball_query: Query<&mut Ball>,
    paddle_query: Query<(&Paddle, &Transform)>,
) {
    if let Ok(mut ball) = ball_query.single_mut() {
        if mouse_button_input.pressed(MouseButton::Left) && ball.velocity == Vec3::default() {
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
        let ininial_position = transform.translation;
        transform.translation += ball.velocity * timer.delta_seconds();
        let final_position = transform.translation;
        println!(
            "travel (x: {}, y: {}); speed: {:?}",
            (ininial_position.x - final_position.x).abs(),
            (ininial_position.y - final_position.y).abs(),
            ball.velocity
        )
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
    diag: Res<Diagnostics>,
) {
    if let Ok((mut ball, ball_transform, ball_sprite)) = ball_query.single_mut() {
        // println!(
        //     "{:?}",
        //     diag.get_measurement(FrameTimeDiagnosticsPlugin::FPS)
        // );
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
