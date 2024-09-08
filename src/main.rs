use bevy::{
    prelude::*, 
    render::camera::ScalingMode, 
    window::{WindowMode, WindowResolution},
};

mod animation;
mod collision;
mod starship;
mod alien;
mod alien_matrix;
mod bullet;

use animation::execute_animation;
use collision::bullet_x_allien_collision;
use starship::{Starship, STARSHIP_POS, STARSHIP_SCALE};
use alien::Alien;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

fn setup(
    mut commands: Commands,
    starship_texture: Res<starship::Texture>
) {
    // Camera setup. Game renders at 1920x1080 and stretch to any window size
    commands.spawn(
        Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.0, 
                near: -1000.0,
                scaling_mode: ScalingMode::Fixed{width: 1920.0, height: 1080.0},
                ..default()
            },
            ..default()
        }
    );

    // Starship: require(texture)
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: STARSHIP_POS,
                scale: STARSHIP_SCALE,
                ..default()
            },
            texture: starship_texture.0.clone(),
            ..default()
        },
        Starship
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Space invaders"),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }
        ))
        .add_systems(PreStartup, ( // load resources
            bullet::load_cooldown_timer,
            bullet::load_texture,
            starship::load_texture,
            alien::load_texture_atlas,
            alien_matrix::load_spawn_cooldown_timer,
            alien_matrix::load_matrix_state
        ).chain())
        .add_systems(Startup, setup) 
        .add_systems(Update, (
            bullet_x_allien_collision,
            execute_animation::<Alien>,
            bullet::tick_spawn_timer,
            alien_matrix::tick_spawn_cooldown_timer,
        ).chain())
        .add_systems(FixedUpdate, ( // load game logic
            starship::mv, 
            bullet::shoot,
            bullet::mv,
            bullet::delete,
            alien_matrix::spawn_aliens
        ).chain())
        .run();
}