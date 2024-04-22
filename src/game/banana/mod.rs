use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use crate::AppState;

use self::{resources::*, systems::*};

use super::SimulationState;

pub const BANANA_SIZE: f32 = 64.0;
pub const NUMBER_OF_BANANAS: usize = 10;

pub struct BananaPlugin;

impl Plugin for BananaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BananaSpawnTimer>()
            .add_system(spawn_bananas.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (tick_banana_spawn_timer, spawn_bananas_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_bananas.in_schedule(OnExit(AppState::Game)));
    }
}
