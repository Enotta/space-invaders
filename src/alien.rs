use bevy::prelude::*;

pub const ALIEN_SIZE: UVec2 = UVec2::new(35, 26);
pub const ALIEN_SCALE: Vec3 = Vec2::new(1.0, 1.0).extend(1.0);

/// Alien unit
#[derive(Component)]
pub struct Alien;

/// Alien texture (two sprites). Load from /assets
#[derive(Resource)]
pub struct Texture(pub Handle<Image>);

/// Load texture from /assets
pub fn load_texture_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let alien_texturs_atlas: Handle<Image> = asset_server.load("alien_atlas.png");
    commands.insert_resource(Texture(alien_texturs_atlas));
}

