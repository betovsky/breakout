use super::{config::CONFIG, Ball, Disposable};
use crate::{loading::BrickAssets, GameState};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::{distributions::Uniform, prelude::Distribution};

struct Brick {
    life: u32,
}

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_board.system()));

        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .after("movement")
                .with_system(brick_collision.system()),
        );
    }
}

fn setup_board(
    mut commands: Commands,
    // materials: Res<MaterialsAssets>,
    brick_assets: Res<BrickAssets>,
) {
    println!("setup game bricks");

    // bricks
    let between = Uniform::from(0..3u32);
    let mut rng = rand::thread_rng();
    let bricks_per_row = (CONFIG.play_area.width / CONFIG.brick_size.width) as i32;
    let brick_width = CONFIG.brick_size.width;
    let brick_height = CONFIG.brick_size.height;
    for row_index in 0..CONFIG.brick_rows {
        let ri = row_index as f32 + 0.5; // count half a block since bevy 0,0 is in the middle of the block
        let starting_height = CONFIG.play_area.height / 2.0 - ri * brick_height;
        for column_index in 0..bricks_per_row {
            let ci = column_index as f32 + 0.5; // count half a block since bevy 0,0 is in the middle of the block
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
                            ci * brick_width - CONFIG.play_area.width / 2.0,
                            starting_height,
                            1.0,
                        ),
                        scale: Vec3::new(brick_width / 200.0, brick_height / 70.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Brick { life: brick_life })
                .insert(Disposable);
        }
    }
}

fn brick_collision(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    mut brick_query: Query<(Entity, &mut Brick, &Transform, &mut TextureAtlasSprite)>,
    mut commands: Commands,
) {
    if let Ok((mut ball, ball_transform, ball_sprite)) = ball_query.single_mut() {
        let brick_size = Vec2::new(CONFIG.brick_size.width, CONFIG.brick_size.height);
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
