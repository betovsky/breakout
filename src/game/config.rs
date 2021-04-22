use bevy::math::Size;

pub struct GameConfig {
    pub play_area: Size,
    pub ball_starting_speed: f32,
    pub wall_thickness: f32,
    pub paddle_starting_size: Size,
    pub brick_size: Size,
    pub brick_rows: i32,
}

pub const CONFIG: GameConfig = GameConfig {
    play_area: Size {
        width: 600.0,
        height: 800.0,
    },
    ball_starting_speed: 300.0,
    wall_thickness: 14.0,
    paddle_starting_size: Size {
        width: 140.0,
        height: 20.0,
    },
    brick_size: Size {
        width: 40.0,
        height: 20.0,
    },
    brick_rows: 10,
};
