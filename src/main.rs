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
mod border;

use animation::execute_animation;
use building::{Building, BuildingTexture};
use bullet::{Bullet, BulletCooldown, BulletTexture};
use collision::{alien_x_building_collision, bullet_x_allien_collision, bullet_x_building_collision};
use alien::{Alien, AlienTextureAtlas};
use starship::{Starship, StarshipTexture};
use border::{Border, BorderTexture};

const LOGIC_WIDTH: f32 = 960.0;
const LOGIC_HEIGHT: f32 = 1080.0;

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
                scaling_mode: ScalingMode::Fixed{width: LOGIC_WIDTH, height: LOGIC_HEIGHT},
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
                    resolution: WindowResolution::new(LOGIC_WIDTH, LOGIC_HEIGHT),
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }
        ))
        .add_systems(PreStartup, ( // load resources (image sprites, atlases, logic timers)
            BulletCooldown::load,
            BulletTexture::load,
            StarshipTexture::load,
            AlienTextureAtlas::load,
            AlienSpawnCooldown::load,
            alien_matrix::MatrixState::load,
            BuildingTexture::load,
            score::load_scores,
            BorderTexture::load
        ).chain())
        .add_systems(Startup, ( // spawn basic objects
            setup_camera,
            Border::load,
            Starship::spawn,
            Building::spawn,
            score::CurrentScore::spawn
        ).chain()) 
        .add_systems(Update, ( // tick logic timers, check collisions, run const animations
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