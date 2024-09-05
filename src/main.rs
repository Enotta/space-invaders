use bevy::{
    prelude::*, render::camera::ScalingMode, window::{WindowMode, WindowResolution}
};

mod animation;
mod starship;
mod alien;
mod bullet;

use starship::{Starship, STARSHIP_POS, STARSHIP_SCALE};
use alien::{Alien, ALIEN_SCALE, ALIEN_SIZE};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    starship_texture: Res<starship::Texture>,
    alien_texture_atlas: Res<alien::Texture>
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
    
    // Aliens: require(texture_atlas, animation)
    let alien_layout = TextureAtlasLayout::from_grid(ALIEN_SIZE, 2, 1, None, None);
    let alien_texture_atlas_layout = texture_atlas_layouts.add(alien_layout);
    let alien_animation_config = animation::Config::new(0, 1, 2);
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: ALIEN_SCALE,
                ..default()
            },
            texture: alien_texture_atlas.0.clone(),
            ..default()
        },
        TextureAtlas {
            layout: alien_texture_atlas_layout.clone(),
            index: alien_animation_config.first_frame
        },
        Alien,
        alien_animation_config
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
            alien::load_texture_atlas
        ).chain())
        .add_systems(Startup, setup) 
        .add_systems(Update, (
            animation::execute_animation::<Alien>
        ).chain())
        .add_systems(FixedUpdate, ( // load game logic
            starship::mv, 
            bullet::tick_spawn_time,
            bullet::shoot,
            bullet::mv,
            bullet::delete
        ).chain())
        .run();
}