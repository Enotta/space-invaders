use bevy::prelude::*;

use crate::alien_matrix::MatrixState;
use crate::bullet::Bullet;
use crate::alien::{Alien, ALIEN_SIZE};
use crate::alien_matrix::FullnessState;

/// Despawn bullet and alien on collision
pub fn bullet_x_allien_collision(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut aliens: Query<(Entity, &Transform), With<Alien>>,
    mut empty_check: ResMut<MatrixState>
) {
    let mut aliens_num = aliens.iter().len();

    for (bullet, bullet_pos ) in bullets.iter_mut() {
        for (alien, alien_pos) in aliens.iter_mut() {
            if intersect(bullet_pos.translation, alien_pos.translation) {
                commands.entity(bullet).despawn();
                commands.entity(alien).despawn();
                aliens_num -= 1;

                if aliens_num == 0 {
                    empty_check.0 = FullnessState::Empty;
                }

                break;
            }
        }
    }
}

/// Check if the bullet and the alien intersects
fn intersect(
    bullet_pos: Vec3,
    alien_pos: Vec3
) -> bool {
    if bullet_pos.x >= alien_pos.x - (ALIEN_SIZE.x as f32) / 2.0 && bullet_pos.x <= alien_pos.x + (ALIEN_SIZE.x as f32) / 2.0 {
        if bullet_pos.y >= alien_pos.y - (ALIEN_SIZE.y as f32) / 2.0 && bullet_pos.y <= alien_pos.y + (ALIEN_SIZE.y as f32) / 2.0 {
            return true;
        }
    }

    false
}