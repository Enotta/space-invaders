use bevy::prelude::*;

mod starship;
mod alien;
mod bullet;

const STARSHIP_SIZE: Vec3 = Vec2::new(40.0, 40.0).extend(1.0);
const STARSHIP_POS: Vec3 = Vec3::new(0.0, -300.0, 0.0);

const ALIEN_SIZE: Vec3 = Vec2::new(40.0, 30.0).extend(1.0);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Starship
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: STARSHIP_POS,
                scale: STARSHIP_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: Color::BLACK,
                ..default()
            },
            ..default()
        },
        starship::Body
    ));

    // Aliens
    for i in 0..1 { let _a = &i * ALIEN_SIZE.x as i32; }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (starship::mv, 
                                                     bullet::shoot,
                                                     bullet::mv,
                                                     bullet::delete).chain())
        .run();
}