use bevy::prelude::*;

use crate::alien::{Alien, Texture, ALIEN_SIZE, ALIEN_SCALE};
use crate::animation;

const ALIEN_MATRIX_WIDTH: usize = 12;
const ALIEN_MATRIX_HEIGHT: usize = 4;
pub const ALIEN_SPAWN_COOLDOWN: f32 = 0.5;
const GAP_X: f32 = 12.0;
const GAP_Y: f32 = 8.0;

/// Cooldown before next line of aliens can appear during spawning
#[derive(Resource)]
pub struct SpawnCooldown(pub Timer);

/// Insert cooldown timer before next lain of aliens may appear
pub fn load_spawn_cooldown_timer(
    mut commands: Commands
) {
    let timer = Timer::from_seconds(ALIEN_SPAWN_COOLDOWN, TimerMode::Once);
    commands.insert_resource(SpawnCooldown(timer));
}

/// Tick timet for aliens' spawn
pub fn tick_spawn_cooldown_timer(
    mut timer: ResMut<SpawnCooldown>,
    time: Res<Time>
) {
    timer.0.tick(time.delta());
}

/// Is matrix empty or no
#[derive(Resource)]
pub struct EmptyCheck(pub bool);

/// Insert state. In the beggining matrix is empty
pub fn load_matrix_state(
    mut commands: Commands
) {
    commands.insert_resource(EmptyCheck(true));
}

/// Spawn new lines of aliens
pub fn spawn_aliens(
    mut commands: Commands,
    aliens: Query<&Alien>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    alien_texture_atlas: Res<Texture>,
    mut timer: ResMut<SpawnCooldown>,
    check: Res<EmptyCheck>
) {
    if timer.0.finished() && check.0 {
        let alien_layout = TextureAtlasLayout::from_grid(ALIEN_SIZE, 2, 1, None, None);
        let alien_texture_atlas_layout = texture_atlas_layouts.add(alien_layout);
        let alien_animation_config = animation::Config::new(0, 1, 2);

        let mut row = ALIEN_MATRIX_HEIGHT;
        let mut rows_empty = [true; ALIEN_MATRIX_HEIGHT];
        for alien in aliens.iter() {
            rows_empty[alien.get_row()] = false;
        }

        for u in 0..rows_empty.len() {
            if rows_empty[u] {
                row = u;
                break;
            }
        }

        if row != ALIEN_MATRIX_HEIGHT {
            for col in 0..ALIEN_MATRIX_WIDTH {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(-((ALIEN_SIZE.x as f32)*6.0 + 5.5*GAP_X) + (col as f32)*(ALIEN_SIZE.x as f32) + (col as f32)*GAP_X + (ALIEN_SIZE.x as f32)/2.0,
                                                ((ALIEN_SIZE.y as f32)*2.0 + 1.5*GAP_Y) - (row as f32)*(ALIEN_SIZE.y as f32) - (row as f32)*GAP_Y - (ALIEN_SIZE.y as f32)/2.0, 
                                                0.0),
                            scale: ALIEN_SCALE,
                            ..default()
                        },
                        texture: alien_texture_atlas.0.clone(),
                        ..default()
                    },
                    TextureAtlas {
                        layout: alien_texture_atlas_layout.clone(),
                        index: alien_animation_config.first_frame.clone()
                    },
                    Alien::new(row, col),
                    alien_animation_config.clone()
                ));
            }
        }

        timer.0.reset();
    }
}
