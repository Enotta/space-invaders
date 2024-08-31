use bevy::prelude::*;

use crate::starship;

const BULLET_SIZE: Vec3 = Vec2::new(3.0, 5.0).extend(1.0);
const BULLET_VELOCITY: f32 = 12.0;

#[derive(Component)]
pub struct Unit;

pub fn shoot(keyboard_input: Res<ButtonInput<KeyCode>>,
             query: Query<&Transform, With<starship::Body>>,
             mut commands: Commands) {
    let starship_pos = query.single();

    if keyboard_input.pressed(KeyCode::Space){
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: starship_pos.translation,
                    scale: BULLET_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                ..default()
            },
            Unit
        ));
    }
}

pub fn mv(mut query: Query<&mut Transform, With<Unit>>) {
    query.iter_mut().for_each(|mut bullet_pos| {
        bullet_pos.translation.y += BULLET_VELOCITY;
    })
}

pub fn delete(query: Query<(Entity, &Transform), With<Unit>>,
              mut commands: Commands) {
    for (bullet, bullet_pos) in query.iter() {
        if bullet_pos.translation.y > 300.0{
            commands.entity(bullet).despawn();
        }
    }
}