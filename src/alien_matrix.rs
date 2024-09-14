use bevy::prelude::*;

use crate::alien::{Alien, Texture, ALIEN_SCALE, ALIEN_SIZE};
use crate::alien_spawn_cooldown::SpawnCooldown;
use crate::animation;

pub const ALIEN_MATRIX_WIDTH: usize = 12;
const ALIEN_MATRIX_HEIGHT: usize = 4;
pub const GAP_X: f32 = 12.0;
pub const GAP_Y: f32 = 8.0;

#[derive(PartialEq)]
pub enum FullnessState {
    Empty,
    Filling,
    Full
}

/// Is matrix empty or no
#[derive(Resource)]
pub struct MatrixState(pub FullnessState);

/// Insert state. In the beggining matrix is empty
pub fn load_matrix_state(
    mut commands: Commands
) {
    commands.insert_resource(MatrixState(FullnessState::Empty));
}

/// Spawn new lines of aliens
pub fn spawn_matrix(
    mut commands: Commands,
    aliens: Query<&Alien>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    alien_texture_atlas: Res<Texture>,
    mut timer: ResMut<SpawnCooldown>,
    mut check: ResMut<MatrixState>
) {
    if timer.0.finished() && (check.0 == FullnessState::Empty || check.0 == FullnessState::Filling) {
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

            check.0 = FullnessState::Filling;
        }
        else {
            check.0 = FullnessState::Full;
        }

        timer.0.reset();
    }
}
