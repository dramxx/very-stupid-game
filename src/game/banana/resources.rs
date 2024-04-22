use bevy::prelude::*;

pub const BANANA_SPAWN_INTERVAL: f32 = 1.0;

#[derive(Resource)]
pub struct BananaSpawnTimer {
    pub timer: Timer,
}

impl Default for BananaSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BANANA_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}
