use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use self::{resources::*, systems::*};
use super::SimulationState;
use crate::AppState;

pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub enum EnemySystemSet {
    Movement,
    Restriction,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(EnemySystemSet::Movement.before(EnemySystemSet::Restriction))
            .init_resource::<EnemySpawnTimer>()
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    enemy_movement.in_set(EnemySystemSet::Movement),
                    restrict_enemy_movement.in_set(EnemySystemSet::Restriction),
                    update_enemy_direction,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
