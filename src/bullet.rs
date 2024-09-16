use bevy::prelude::*;

use crate::score::CurrentScore;
use crate::starship::Starship;
use crate::alien_matrix::{FullnessState, MatrixState};

const BULLET_SCALE: Vec3 = Vec3::new(2.0, 2.0, 1.0);
const BULLET_VELOCITY: f32 = 11.0;
const BULLET_COOLDOWN_TIME: f32 = 0.25;

/// Bullet entity. Shoot from starship on KeyCode::Space
#[derive(Component)]
pub struct Bullet;

impl Bullet {
    /// Shoot bullets on Space or hold Space
    pub fn shoot(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        query: Query<&Transform, With<Starship>>,
        mut commands: Commands,
        mut cooldown: ResMut<BulletCooldown>, 
        bullet_texture: Res<BulletTexture>,
        matrix_state: Res<MatrixState>
    ) {
        if query.iter().len() == 0 { return; }
        let starship_pos = query.single();

        if keyboard_input.pressed(KeyCode::Space) && cooldown.0.finished() && matrix_state.0 != FullnessState::Filling {
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
                Self
            ));

            cooldown.0.reset();
        }
    }

    /// Move bullets on the screen at BULLET_VELOCITY
    pub fn mv(
        mut query: Query<&mut Transform, With<Self>>
    ) {
        query.iter_mut().for_each(|mut bullet_pos| {
            bullet_pos.translation.y += BULLET_VELOCITY;
        })
    }

    /// Delete bullet when goes out from scope. Decrease score if bullet misses
    pub fn despawn(
        query: Query<(Entity, &Transform), With<Self>>,
        mut commands: Commands,
        mut cur_score: ResMut<CurrentScore>
    ) {
        for (bullet, bullet_pos) in query.iter() {
            if bullet_pos.translation.y > crate::WINDOW_HEIGHT / 2.1 {
                commands.entity(bullet).despawn();
                if cur_score.0 > 0 {
                    cur_score.0 -= 10;
                }
            }
        }
    }
}

/// Cooldown before next bullet can be ejected
#[derive(Resource)]
pub struct BulletCooldown(pub Timer);

impl BulletCooldown {
    /// Start bullet cooldown timer
    pub fn load(
        mut commands: Commands
    ) {
        let timer = Timer::from_seconds(BULLET_COOLDOWN_TIME, TimerMode::Once);
        commands.insert_resource(Self(timer));
    }

    /// Tick timer on Update
    pub fn tick(
        time: Res<Time>,
        mut cooldown: ResMut<Self>
    ) {
        cooldown.0.tick(time.delta());
    }
}

/// Bullet texture. Stored in /assets
#[derive(Resource)]
pub struct BulletTexture(Handle<Image>);

impl BulletTexture {
    /// Load bullet texture from /assets
    pub fn load(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        let bullet_texture: Handle<Image> = asset_server.load("bullet.png");
        commands.insert_resource(BulletTexture(bullet_texture));
    }
}
