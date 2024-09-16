use bevy::prelude::*;

const BUILDINGS_COUNT: usize = 4;
pub const BUILDING_SIZE: UVec2 = UVec2::new(90, 66);
const BUILDING_SCALE: Vec3 = Vec3::new(3.0,  3.0, 1.0);
const GAP_X: f32 = 40.0;

/// Building entity
#[derive(Component)]
pub struct Building {
    pub state: isize
}

impl Building {
    /// Creates new building with max health
    pub fn new() -> Self {
        Self {
            state: 3
        }
    }

    /// Spawn row of undamaged buildings
    pub fn spawn(
        mut commands: Commands,
        texture: Res<BuildingTexture>
    ) {
        for i in 0..BUILDINGS_COUNT {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new((BUILDINGS_COUNT as f32) * (BUILDING_SIZE.x as f32) / 2.0 + (((BUILDINGS_COUNT as f32) / 2.0).round() - 0.5) * GAP_X - (i as f32 + 0.5) * (BUILDING_SIZE.x as f32) - (i as f32) * GAP_X, -300.0, 0.0),
                        scale: BUILDING_SCALE,
                        ..default()
                    },
                    texture: texture.3.clone(),
                    ..default()
                },
                Self::new()
            ));
        }
    }
}

/// Building textures for each of four stages
#[derive(Resource)]
pub struct BuildingTexture(pub Handle<Image>, pub Handle<Image>, pub Handle<Image>, pub Handle<Image>);

impl BuildingTexture {
    /// Load building textures from /assets
    pub fn load(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {
        let house0: Handle<Image> = asset_server.load("house0.png");
        let house1: Handle<Image> = asset_server.load("house1.png");
        let house2: Handle<Image> = asset_server.load("house2.png");
        let house3: Handle<Image> = asset_server.load("house3.png");

        commands.insert_resource(Self(house0, house1, house2, house3));
    }
}
