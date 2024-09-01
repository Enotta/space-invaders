use bevy::prelude::*;

const STARSHIP_VELOCITY: f32 = 5.0;

#[derive(Component)]
pub struct Body;

#[derive(Resource)]
pub struct Texture(pub Handle<Image>);

pub fn load_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let starship_texture: Handle<Image> = asset_server.load("starship.png");
    commands.insert_resource(Texture(starship_texture));
}

pub fn mv(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Body>>
) {
    let mut starship_pos = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        starship_pos.translation.x -= STARSHIP_VELOCITY;
    }
    else if keyboard_input.pressed(KeyCode::KeyD) ||  keyboard_input.pressed(KeyCode::ArrowRight) {
        starship_pos.translation.x += STARSHIP_VELOCITY;
    }
}