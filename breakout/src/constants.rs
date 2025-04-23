use bevy::{color::Color, math::{Vec2, Vec3}, ui::Val};


pub const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
pub const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
pub const PADDLE_SPEED: f32 = 500.0;

pub const PADDLE_PADDING: f32 = 10.0;


pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
pub const BALL_DIAMETER: f32 = 30.;
pub const BALL_SPEED: f32 = 400.0;
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

pub const WALL_THICKNESS: f32 = 10.0;

pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;

pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

pub const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);

pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
pub const GAP_BETWEEN_BRICKS: f32 = 5.0;

pub const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
pub const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

pub const SCOREBOARD_FONT_SIZE: f32 = 33.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

pub const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
pub const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
pub const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
pub const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
pub const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
pub const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
