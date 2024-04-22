use bevy::prelude::*;

pub mod components;
mod systems;

use crate::AppState;

use self::systems::*;

use super::SimulationState;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub enum PlayerSystemSet {
    Movement,
    Restriction,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Restriction))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    restrict_player_movement.in_set(PlayerSystemSet::Movement),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (player_hit_banana, enemy_hit_player)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
