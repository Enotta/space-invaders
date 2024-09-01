use bevy::{prelude::*, window::WindowResolution};

mod starship;
mod alien;
mod bullet;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

const STARSHIP_SIZE: Vec3 = Vec2::new(1.0, 1.0).extend(1.0);
const STARSHIP_POS: Vec3 = Vec3::new(0.0, -400.0, 0.0);

const ALIEN_SIZE: Vec3 = Vec2::new(40.0, 30.0).extend(1.0);

fn setup(
    mut commands: Commands, starship_texture: Res<starship::Texture>
) {
    commands.spawn(Camera2dBundle::default());

    // Starship
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: STARSHIP_POS,
                scale: STARSHIP_SIZE,
                ..default()
            },
            texture: starship_texture.0.clone(),
            ..default()
        },
        starship::Body
    ));

    // Aliens
    for i in 0..1 { let _a = &i * ALIEN_SIZE.x as i32; }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Space invaders".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_systems(PreStartup, ( // load resources
            bullet::load_cooldown_timer,
            bullet::load_texture,
            starship::load_texture
        ).chain())
        .add_systems(Startup, setup) 
        .add_systems(FixedUpdate, ( // load game logic
            starship::mv, 
            bullet::tick_spawn_time,
            bullet::shoot,
            bullet::mv,
            bullet::delete
        ).chain())
        .run();
}