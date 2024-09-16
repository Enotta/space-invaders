use bevy::prelude::*;

use crate::alien_matrix::MatrixState;
use crate::building::{Building, BuildingTexture, BUILDING_SIZE};
use crate::bullet::Bullet;
use crate::alien::{Alien, ALIEN_SIZE};
use crate::alien_matrix::FullnessState;
use crate::score::CurrentScore;

/// Despawn bullet and alien on collision
pub fn bullet_x_allien_collision(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut aliens: Query<(Entity, &Transform), With<Alien>>,
    mut empty_check: ResMut<MatrixState>,
    mut cur_score: ResMut<CurrentScore>
) {
    let mut aliens_num = aliens.iter().len();

    for (bullet, bullet_pos ) in bullets.iter_mut() {
        for (alien, alien_pos) in aliens.iter_mut() {
            if bullet_x_alien_intersect(bullet_pos.translation, alien_pos.translation) {
                commands.entity(bullet).despawn();
                commands.entity(alien).despawn();
                aliens_num -= 1;
                cur_score.0 += 50;

                break;
            }
        }
    }

    if aliens_num == 0 {
        empty_check.0 = FullnessState::Empty;
    }
}

/// Check if the bullet and the alien intersects
fn bullet_x_alien_intersect(
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

/// Despawn bullet and alien on collision
pub fn bullet_x_building_collision(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    buidings: Query<&Transform, With<Building>>
) {
    for (bullet, bullet_pos ) in bullets.iter_mut() {
        for building_pos in buidings.iter() {
            if bullet_x_building_intersect(bullet_pos.translation, building_pos.translation) {
                commands.entity(bullet).despawn();
                break;
            }
        }
    }
}

/// Check if the bullet and the building intersects
fn bullet_x_building_intersect(
    bullet_pos: Vec3,
    building_pos: Vec3
) -> bool {
    if bullet_pos.x >= building_pos.x - (BUILDING_SIZE.x as f32) / 2.0 && bullet_pos.x <= building_pos.x + (BUILDING_SIZE.x as f32) / 2.0 {
        if bullet_pos.y >= building_pos.y - (BUILDING_SIZE.y as f32) / 2.0 && bullet_pos.y <= building_pos.y + (BUILDING_SIZE.y as f32) / 2.0 {
            return true;
        }
    }

    false
}

/// Despawn aliens on collision with building. Crush building on collision
pub fn alien_x_building_collision(
    mut commands: Commands,
    mut aliens: Query<(Entity, &Transform), With<Alien>>,
    mut buildings: Query<(Entity, &Transform, &mut Building, &mut Handle<Image>), With<Building>>,
    building_textures: Res<BuildingTexture>,
    mut empty_check: ResMut<MatrixState>
) {
    let mut aliens_num = aliens.iter().len();

    for (alien, alien_pos) in aliens.iter_mut() {
        if alien_pos.translation.y < -300.0 {
            commands.entity(alien).despawn();
            aliens_num -= 1;
        }

        for (building, building_pos, mut building_conf, mut building_texture) in buildings.iter_mut() {
            if alien_x_building_intersect(alien_pos.translation, building_pos.translation)  {
                commands.entity(alien).despawn();
                aliens_num -= 1;
                
                building_conf.state -= 1;
                if building_conf.state == -1 {
                    commands.entity(building).despawn();
                }
                else if building_conf.state == 0 { 
                    *building_texture = building_textures.0.clone();
                }
                else if building_conf.state == 1 { 
                    *building_texture = building_textures.1.clone();
                }
                else if building_conf.state == 2 { 
                    *building_texture = building_textures.2.clone();
                }

                break;
            }
        }
    }

    if aliens_num == 0 {
        empty_check.0 = FullnessState::Empty;
    }
}

/// Check if alien intersects with building
fn alien_x_building_intersect(
    alien_pos: Vec3, 
    building_pos: Vec3
) -> bool {
    if alien_pos.x >= building_pos.x - (BUILDING_SIZE.x as f32) / 2.0 && alien_pos.x <= building_pos.x + (BUILDING_SIZE.x as f32) / 2.0 {
        if alien_pos.y >= building_pos.y - (BUILDING_SIZE.y as f32) / 2.0 && alien_pos.y <= building_pos.y + (BUILDING_SIZE.y as f32) / 2.0 {
            return true;
        }
    }

    false
}