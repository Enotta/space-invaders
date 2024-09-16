use bevy::prelude::*;

use crate::alien_matrix::{FullnessState, MatrixState, ALIEN_MATRIX_WIDTH, GAP_X, GAP_Y};

pub const ALIEN_SIZE: UVec2 = UVec2::new(35, 26);
pub const ALIEN_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const ALIEN_VELOCITY: f32 = 1.2;
const LEFT_ABSOLUTE_BORDER: f32 = -400.0;
const RIGHT_ABSOLUTE_BORDER: f32 = 400.0;

#[derive(Clone, Copy)]
pub enum AlienMovementDirection {
    Right,
    Left
}

/// Alien unit
#[derive(Component)]
pub struct Alien {
    row: usize,
    col: usize,
    dir: AlienMovementDirection
}

impl Alien {
    /// Creates new alien at given pos
    pub fn new(
        r: usize, 
        c: usize
    ) -> Self {
        Self {
            row: r,
            col: c,
            dir: AlienMovementDirection::Left
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

    /// Get alien movement direction
    pub fn get_dir(&self) -> AlienMovementDirection {
        self.dir
    }

    /// Move aliens on the screen on matrix state condition
    pub fn mv(
        mut query: Query<(&mut Transform, &mut Self)>,
        matrix_state: Res<MatrixState>
    ) {
        if matrix_state.0 == FullnessState::Full {
            for (mut alien_pos, mut alien) in query.iter_mut() {
                match alien.dir {
                    AlienMovementDirection::Left => {alien_pos.translation.x -= ALIEN_VELOCITY},
                    AlienMovementDirection::Right => {alien_pos.translation.x += ALIEN_VELOCITY}
                }
        
                if alien_pos.translation.x < LEFT_ABSOLUTE_BORDER + ((alien.get_col() as f32) + 0.5) * (ALIEN_SIZE.x as f32) + ((alien.get_col() as f32)) * (GAP_X as f32) {
                    alien.dir = AlienMovementDirection::Right;
                    alien_pos.translation.y -= (ALIEN_SIZE.y as f32) + GAP_Y;
                }
                else if alien_pos.translation.x > RIGHT_ABSOLUTE_BORDER - ((ALIEN_MATRIX_WIDTH as f32) - (alien.get_col() as f32) + 0.5) * (ALIEN_SIZE.x as f32) - ((ALIEN_MATRIX_WIDTH as f32) - (alien.get_col() as f32)) * (GAP_X as f32) {
                    alien.dir = AlienMovementDirection::Left;
                    alien_pos.translation.y -= (ALIEN_SIZE.y as f32) + GAP_Y;
                }
            }
        }
    }
}

/// Alien texture (two sprites). Load from /assets
#[derive(Resource)]
pub struct AlienTextureAtlas(pub Handle<Image>);

impl AlienTextureAtlas {
    /// Load texture from /assets
    pub fn load(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        let alien_texturs_atlas: Handle<Image> = asset_server.load("alien_atlas.png");
        commands.insert_resource(Self(alien_texturs_atlas));
    }
}
