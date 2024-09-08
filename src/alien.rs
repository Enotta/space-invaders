use bevy::prelude::*;

pub const ALIEN_SIZE: UVec2 = UVec2::new(35, 26);
pub const ALIEN_SCALE: Vec3 = Vec2::new(1.0, 1.0).extend(1.0);

/// Alien unit
#[derive(Component, Debug)]
pub struct Alien {
    row: usize,
    col: usize
}

impl Alien {
    /// Creates new alien at given pos
    pub fn new(
        r: usize, 
        c: usize
    ) -> Self {
        Self {
            row: r,
            col: c
        }
    }

    /// Get alien row number
    pub fn get_row(&self) -> usize {
        self.row
    }

    /// Get alien col number
    pub fn get_col(&self) -> usize {
        self.col
    }
}

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
