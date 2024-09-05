use bevy::prelude::*;

const STARSHIP_VELOCITY: f32 = 6.0;
const STARSHIP_SIZE: Vec2 = Vec2::new(40.0, 40.0);
pub const STARSHIP_SCALE: Vec3 = Vec2::new(1.0, 1.0).extend(1.0);
pub const STARSHIP_POS: Vec3 = Vec3::new(0.0, -400.0, 0.0);

/// Starship entity. Presented in single unit
#[derive(Component)]
pub struct Starship;

/// Starship texture. Load from /assets
#[derive(Resource)]
pub struct Texture(pub Handle<Image>);

/// Load starship texture from /assets
pub fn load_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let starship_texture: Handle<Image> = asset_server.load("starship.png");
    commands.insert_resource(Texture(starship_texture));
}

/// Move starship. Limited by window borders
pub fn mv(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Starship>>
) {
    let mut starship_pos = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        if starship_pos.translation.x > (-crate::WINDOW_WIDTH + STARSHIP_SIZE.x) / 2.0 {
            starship_pos.translation.x -= STARSHIP_VELOCITY;
        }
    }
    else if keyboard_input.pressed(KeyCode::KeyD) ||  keyboard_input.pressed(KeyCode::ArrowRight) {
        if starship_pos.translation.x < (crate::WINDOW_WIDTH - STARSHIP_SIZE.x) / 2.0 {
            starship_pos.translation.x += STARSHIP_VELOCITY;
        }
    }
}