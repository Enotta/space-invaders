use std::collections::HashMap;

use bevy::prelude::*;

use crate::alien::{Alien, Texture, ALIEN_SIZE, ALIEN_SCALE};
use crate::alien_position::AlienPos;
use crate::animation;

const ALIEN_MATRIX_WIDTH: usize = 12;
const ALIEN_MATRIX_HEIGHT: usize = 4;
pub const ALIEN_SPAWN_COOLDOWN: f32 = 0.5;
const GAP_X: f32 = 12.0;
const GAP_Y: f32 = 8.0;

/// Alien matrix. Responsible for spawning logic
#[derive(Component)]
pub struct AlienMatrix(pub HashMap<AlienPos, Option<Alien>>);

/// Cooldown before next line of aliens can appear during spawning
#[derive(Resource)]
pub struct SpawnCooldown(pub Timer);

/// Is matrix empty or no
#[derive(Resource)]
pub struct CheckState(bool);

fn create_alien_matrix() -> HashMap<AlienPos, Option<Alien>> {
    let mut matrix = HashMap::new();

    for row in 0..ALIEN_MATRIX_HEIGHT {
        for col in 0..ALIEN_MATRIX_WIDTH {
            matrix.insert(AlienPos::new(row, col), None);
        }
    }

    matrix
}

/// Spawn alien layout
pub fn load_alien_matrix(
    mut commands: Commands
) {
    let alien_matrix = AlienMatrix(create_alien_matrix());
    commands.spawn(alien_matrix);
}

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

/// Insert state. In the beggining matrix is empty
pub fn load_matrix_state(
    mut commands: Commands
) {
    commands.insert_resource(CheckState(false));
}

/// Spawn new lines of aliens
pub fn spawn_aliens(
    mut commands: Commands,
    mut matrix_query: Query<&mut AlienMatrix>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    alien_texture_atlas: Res<Texture>,
    mut timer: ResMut<SpawnCooldown>,
    check: Res<CheckState>
) {
    if timer.0.finished() && check.0 {
        let mut matrix = matrix_query.single_mut();

        let alien_layout = TextureAtlasLayout::from_grid(ALIEN_SIZE, 2, 1, None, None);
        let alien_texture_atlas_layout = texture_atlas_layouts.add(alien_layout);
        let alien_animation_config = animation::Config::new(0, 1, 2);

        let mut index = ALIEN_MATRIX_HEIGHT;
        for row in 0..ALIEN_MATRIX_HEIGHT {
            let mut spawn_flag = true;

            for col in 0..ALIEN_MATRIX_WIDTH {
                let cur = matrix.0.get(&AlienPos::new(row, col));
                if cur.is_some() && cur.unwrap().is_some() {
                    spawn_flag = false;
                    break;
                }
            }

            if spawn_flag {
                index = row;
                break;
            }
        }

        if index != ALIEN_MATRIX_HEIGHT {
            for col in 0..ALIEN_MATRIX_WIDTH {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(-((ALIEN_SIZE.x as f32)*6.0 + 5.5*GAP_X) + (col as f32)*(ALIEN_SIZE.x as f32) + (col as f32)*GAP_X + (ALIEN_SIZE.x as f32)/2.0,
                                                ((ALIEN_SIZE.y as f32)*2.0 + 1.5*GAP_Y) - (index as f32)*(ALIEN_SIZE.y as f32) - (index as f32)*GAP_Y - (ALIEN_SIZE.y as f32)/2.0, 
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
                    Alien,
                    AlienPos::new(index, col),
                    alien_animation_config.clone()
                ));

                matrix.0.insert(AlienPos::new(index, col), Some(Alien));
            }
        }

        timer.0.reset();
    }
}
