use bevy::prelude::*;

use crate::alien_matrix::AlienMatrix;
use crate::alien_position::AlienPos;
use crate::bullet::Bullet;
use crate::alien::{Alien, ALIEN_SIZE};

/// Despawn bullet and alien on collision
pub fn bullet_x_allien_collision(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut aliens: Query<(Entity, &AlienPos, &Transform), With<Alien>>,
    mut alien_matrix: Query<&mut AlienMatrix>
) {
    for (bullet, bullet_pos ) in bullets.iter_mut() {
        for (alien, alien_place, alien_pos) in aliens.iter_mut() {
            if intersect(bullet_pos.translation, alien_pos.translation) {
                let mut matrix = alien_matrix.single_mut();
                commands.entity(bullet).despawn();
                commands.entity(alien).despawn();
                matrix.0.remove(&AlienPos::new(alien_place.get_row(), alien_place.get_col()));

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