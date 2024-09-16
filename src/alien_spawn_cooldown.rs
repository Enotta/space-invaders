use bevy::prelude::*;

const ALIEN_SPAWN_COOLDOWN: f32 = 0.5;

/// Cooldown before next line of aliens can appear during spawning
#[derive(Resource)]
pub struct AlienSpawnCooldown(pub Timer);

impl AlienSpawnCooldown {
    /// Insert cooldown timer before next lain of aliens may appear
    pub fn load(
        mut commands: Commands
    ) {
        let timer = Timer::from_seconds(ALIEN_SPAWN_COOLDOWN, TimerMode::Once);
        commands.insert_resource(Self(timer));
    }

    /// Tick timet for aliens' spawn
    pub fn tick(
        mut timer: ResMut<Self>,
        time: Res<Time>
    ) {
        timer.0.tick(time.delta());
    }
}
