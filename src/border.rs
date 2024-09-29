use bevy::prelude::*;

use crate::{LOGIC_WIDTH, LOGIC_HEIGHT};

const CELL_SIZE: Vec2 = Vec2::new(40.0, 40.0);

#[derive(Component)]
pub struct Border;

impl Border {
    /// Draw border
    pub fn load(
        mut commands: Commands,
        border_texture: Res<BorderTexture>
    ) {
        let grid_x = (LOGIC_WIDTH/CELL_SIZE.x) as usize;
        let grid_y = (LOGIC_HEIGHT/CELL_SIZE.y) as usize;

        for i in 0..grid_x {
            for j in 0..grid_y {
                if i == 0 || j == 0 || i == grid_x-1 || j == grid_y-1 {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform {
                                translation: Vec3::new(20.0 + (i as f32) * CELL_SIZE.x - LOGIC_WIDTH / 2.0, 20.0 + (j as f32) * CELL_SIZE.y - LOGIC_HEIGHT / 2.0, 0.0),
                                scale: Vec3::new(1.0, 1.0, 1.0),
                                ..default()
                            },
                            texture: border_texture.0.clone(),
                            ..default()
                        },
                        Self
                    ));
                }
            }
        }
    }
}

#[derive(Resource)]
pub struct BorderTexture(pub Handle<Image>);

impl BorderTexture {
    pub fn load(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        let border_texture: Handle<Image> = asset_server.load("border.png");
        commands.insert_resource(Self(border_texture));
    }
}