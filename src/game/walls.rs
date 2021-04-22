use super::{config::CONFIG, Ball};
use crate::{loading::MaterialsAssets, GameState};
use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        Sprite,
    },
};

struct Wall;
pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_walls.system()));

        app.add_system_set(
            SystemSet::on_update(GameState::Game)
                .after("movement")
                .with_system(ball_wall_collision.system()),
        );
    }
}

fn setup_walls(mut commands: Commands, materials: Res<MaterialsAssets>) {
    // left
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.wall.clone(),
            transform: Transform::from_xyz(
                -CONFIG.play_area.width / 2.0 - CONFIG.wall_thickness / 2.0,
                0.0,
                0.0,
            ),
            sprite: Sprite::new(Vec2::new(
                CONFIG.wall_thickness,
                CONFIG.play_area.height + 2.0 * CONFIG.wall_thickness,
            )),
            ..Default::default()
        })
        .insert(Wall);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.wall.clone(),
            transform: Transform::from_xyz(
                CONFIG.play_area.width / 2.0 + CONFIG.wall_thickness / 2.0,
                0.0,
                0.0,
            ),
            sprite: Sprite::new(Vec2::new(
                CONFIG.wall_thickness,
                CONFIG.play_area.height + 2.0 * CONFIG.wall_thickness,
            )),
            ..Default::default()
        })
        .insert(Wall);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.wall.clone(),
            transform: Transform::from_xyz(
                0.0,
                CONFIG.play_area.height / 2.0 + CONFIG.wall_thickness / 2.0,
                0.0,
            ),
            sprite: Sprite::new(Vec2::new(
                CONFIG.play_area.width + 2.0 * CONFIG.wall_thickness,
                CONFIG.wall_thickness,
            )),
            ..Default::default()
        })
        .insert(Wall);
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
