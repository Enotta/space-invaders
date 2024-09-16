use alien_spawn_cooldown::AlienSpawnCooldown;
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
mod alien_spawn_cooldown;
mod bullet;
mod building;
mod game_logic;
mod score;

use animation::execute_animation;
use building::{Building, BuildingTexture};
use bullet::{Bullet, BulletCooldown, BulletTexture};
use collision::{alien_x_building_collision, bullet_x_allien_collision, bullet_x_building_collision};
use alien::{Alien, AlienTextureAtlas};
use starship::{Starship, StarshipTexture};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

/// Set camera config
fn setup_camera(
    mut commands: Commands
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
            BulletCooldown::load,
            BulletTexture::load,
            StarshipTexture::load,
            AlienTextureAtlas::load,
            AlienSpawnCooldown::load,
            alien_matrix::MatrixState::load,
            BuildingTexture::load,
            score::load_scores,
        ).chain())
        .add_systems(Startup, (
            setup_camera,
            Starship::spawn,
            Building::spawn,
            score::CurrentScore::spawn
        ).chain()) 
        .add_systems(Update, (
            bullet_x_allien_collision,
            bullet_x_building_collision,
            alien_x_building_collision,
            execute_animation::<Alien>,
            BulletCooldown::tick,
            AlienSpawnCooldown::tick,
            game_logic::run,
            score::CurrentScore::display
        ).chain())
        .add_systems(FixedUpdate, ( // load game logic
            Starship::mv, 
            Bullet::shoot,
            Bullet::mv,
            Bullet::despawn,
            alien_matrix::spawn_matrix,
            Alien::mv
        ).chain())
        .run();
}