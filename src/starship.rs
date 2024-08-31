use bevy::prelude::*;

const STARSHIP_VELOCITY: f32 = 5.0;

#[derive(Component)]
pub struct Body;

pub fn mv(keyboard_input: Res<ButtonInput<KeyCode>>,
          mut query: Query<&mut Transform, With<Body>>) {

    let mut starship_pos = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        starship_pos.translation.x -= STARSHIP_VELOCITY;
    }
    else if keyboard_input.pressed(KeyCode::KeyD) ||  keyboard_input.pressed(KeyCode::ArrowRight) {
        starship_pos.translation.x += STARSHIP_VELOCITY;
    }
}