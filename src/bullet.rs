use bevy::prelude::*;

use crate::starship;

const BULLET_SIZE: Vec3 = Vec2::new(2.0, 2.0).extend(1.0);
const BULLET_VELOCITY: f32 = 10.0;
pub const BULLET_COOLDOWN_TIME: f32 = 0.25;

#[derive(Component)]
pub struct Unit;

#[derive(Resource)]
pub struct Cooldown(pub Timer);

#[derive(Resource)]
pub struct Texture(Handle<Image>);

pub fn load_cooldown_timer(
    mut commands: Commands
) {
    let timer = Timer::from_seconds(BULLET_COOLDOWN_TIME, TimerMode::Once);
    commands.insert_resource(Cooldown(timer));
}

pub fn load_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    
    let bullet_texture: Handle<Image> = asset_server.load("bullet.png");
    commands.insert_resource(Texture(bullet_texture));
}

pub fn tick_spawn_time(
    time: Res<Time>,
    mut cooldown: ResMut<Cooldown>
) {
    cooldown.0.tick(time.delta());
}

pub fn shoot(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<starship::Body>>,
    mut commands: Commands,
    mut cooldown: ResMut<Cooldown>, 
    bullet_texture: Res<Texture>
) {
    let starship_pos = query.single();

    if keyboard_input.pressed(KeyCode::Space) && cooldown.0.finished() {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: starship_pos.translation,
                    scale: BULLET_SIZE,
                    ..default()
                },
                texture: bullet_texture.0.clone(),
                ..default()
            },
            Unit
        ));

        cooldown.0.reset();
    }
}

pub fn mv(
    mut query: Query<&mut Transform, With<Unit>>
) {
    query.iter_mut().for_each(|mut bullet_pos| {
        bullet_pos.translation.y += BULLET_VELOCITY;
    })
}

pub fn delete(
    query: Query<(Entity, &Transform), With<Unit>>,
    mut commands: Commands
) {
    for (bullet, bullet_pos) in query.iter() {
        if bullet_pos.translation.y > 400.0 {
            commands.entity(bullet).despawn();
        }
    }
}