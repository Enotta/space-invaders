use bevy::prelude::*;

use crate::starship::Starship;

const BULLET_SCALE: Vec3 = Vec2::new(2.0, 2.0).extend(1.0);
const BULLET_VELOCITY: f32 = 11.0;
pub const BULLET_COOLDOWN_TIME: f32 = 0.25;

/// Bullet entity. Shoot from starship on KeyCode::Space
#[derive(Component)]
pub struct Bullet;

/// Cooldown before next bullet can be ejected
#[derive(Resource)]
pub struct Cooldown(pub Timer);

/// Bullet texture. Stored in /assets
#[derive(Resource)]
pub struct Texture(Handle<Image>);

/// Start bullet cooldown timer
pub fn load_cooldown_timer(
    mut commands: Commands
) {
    let timer = Timer::from_seconds(BULLET_COOLDOWN_TIME, TimerMode::Once);
    commands.insert_resource(Cooldown(timer));
}

/// Load bullet texture from /assets
pub fn load_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    
    let bullet_texture: Handle<Image> = asset_server.load("bullet.png");
    commands.insert_resource(Texture(bullet_texture));
}

/// Tick timer on Update
pub fn tick_spawn_timer(
    time: Res<Time>,
    mut cooldown: ResMut<Cooldown>
) {
    cooldown.0.tick(time.delta());
}

/// Shoot bullets on Space or hold Space
pub fn shoot(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Starship>>,
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
                    scale: BULLET_SCALE,
                    ..default()
                },
                texture: bullet_texture.0.clone(),
                ..default()
            },
            Bullet
        ));

        cooldown.0.reset();
    }
}

/// Move bullets on the screen at BULLET_VELOCITY
pub fn mv(
    mut query: Query<&mut Transform, With<Bullet>>
) {
    query.iter_mut().for_each(|mut bullet_pos| {
        bullet_pos.translation.y += BULLET_VELOCITY;
    })
}

/// Delete bullet when goes out from scope 
pub fn delete(
    query: Query<(Entity, &Transform), With<Bullet>>,
    mut commands: Commands
) {
    for (bullet, bullet_pos) in query.iter() {
        if bullet_pos.translation.y > crate::WINDOW_HEIGHT / 2.1 {
            commands.entity(bullet).despawn();
        }
    }
}