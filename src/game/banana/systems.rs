use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::random;

use super::components::*;
use super::resources::*;

use super::NUMBER_OF_BANANAS;

pub fn spawn_bananas(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_BANANAS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/banana/banana.png"),
                ..default()
            },
            Banana {},
        ));
    }
}

pub fn despawn_bananas(mut commands: Commands, banana_query: Query<Entity, With<Banana>>) {
    for banana_entity in banana_query.iter() {
        commands.entity(banana_entity).despawn();
    }
}

pub fn tick_banana_spawn_timer(mut banana_spawn_timer: ResMut<BananaSpawnTimer>, time: Res<Time>) {
    banana_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_bananas_over_time(
    mut commands: Commands,
    banana_spawn_timer: Res<BananaSpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if banana_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/bonus/banana.png"),
                ..default()
            },
            Banana {},
        ));
    }
}
