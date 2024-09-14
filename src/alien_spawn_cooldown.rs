use bevy::prelude::*;

const ALIEN_SPAWN_COOLDOWN: f32 = 0.5;

/// Cooldown before next line of aliens can appear during spawning
#[derive(Resource)]
pub struct SpawnCooldown(pub Timer);

/// Insert cooldown timer before next lain of aliens may appear
pub fn load_spawn_cooldown_timer(
    mut commands: Commands
) {
    let timer = Timer::from_seconds(ALIEN_SPAWN_COOLDOWN, TimerMode::Once);
    commands.insert_resource(SpawnCooldown(timer));
}

/// Tick timet for aliens' spawn
pub fn tick_spawn_cooldown_timer(
    mut timer: ResMut<SpawnCooldown>,
    time: Res<Time>
) {
    timer.0.tick(time.delta());
}