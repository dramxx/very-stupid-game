mod game_menu;
mod hud;
mod pause_menu;

use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HudPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_plugin(GameOverPlugin);
    }
}
