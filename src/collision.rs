use bevy::prelude::*;

use crate::bullet::{Bullet, BULLET_SIZE};
use crate::alien::{Alien, ALIEN_SIZE};

pub fn bullet_x_allien_collision(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut aliens: Query<(Entity, &Transform), With<Alien>>
) {
    for (bullet, bullet_pos ) in bullets.iter_mut() {
        for (alien, alien_pos) in aliens.iter_mut() {
            if intersect(bullet_pos.translation, alien_pos.translation) {
                commands.entity(bullet).despawn();
                commands.entity(alien).despawn();
            }
        }
    }
}

fn intersect(
    bullet_pos: Vec3,
    alien_pos: Vec3
) -> bool {
    if bullet_pos.x + BULLET_SIZE.x / 2.0 >= alien_pos.x - (ALIEN_SIZE.x as f32) / 2.0 && bullet_pos.x - BULLET_SIZE.x / 2.0 <= alien_pos.x + (ALIEN_SIZE.x as f32) / 2.0 {
        if bullet_pos.y + BULLET_SIZE.y / 2.0 >= alien_pos.y - (ALIEN_SIZE.y as f32) / 2.0 && bullet_pos.y - BULLET_SIZE.x / 2.0 <= alien_pos.y + (ALIEN_SIZE.y as f32) / 2.0 {
            return true;
        }
    }

    false
}