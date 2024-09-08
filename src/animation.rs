use std::time::Duration;

use bevy::prelude::*;

/// Animation config
#[derive(Component, Clone)]
pub struct Config {
    pub first_frame: usize,
    pub last_frame: usize,
    pub frame_timer: Timer
}

impl Config {
    /// Create new config setting first and last frame and fps
    pub fn new(frame1: usize, frame2: usize, fps: u8) -> Self {
        Config {
            first_frame: frame1,
            last_frame: frame2,
            frame_timer: Self::get_timer_from_fps(fps)
        }
    }

    /// Converts given fps into timer for one frame to complete
    fn get_timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

/// Tick timer and change frame on timer.just_finished()
pub fn execute_animation<T: Component>(
    time: Res<Time>,
    mut query: Query<(&mut Config, &mut TextureAtlas), With<T>>,
) {
    for (mut config, mut atlas) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if atlas.index == config.last_frame {
                atlas.index = config.first_frame;
            } else {
                atlas.index += 1;
            }
            
            config.frame_timer.reset();
        }
    }
}