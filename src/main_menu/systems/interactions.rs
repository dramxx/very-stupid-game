use bevy::{app::AppExit, prelude::*};

use crate::{
    main_menu::{
        components::*,
        styles::{HOVER_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
    },
    AppState,
};

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = BackgroundColor(PRESSED_BUTTON_COLOR.into());
                app_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(HOVER_BUTTON_COLOR.into());
            }
            Interaction::None => {
                *background_color = BackgroundColor(NORMAL_BUTTON_COLOR.into());
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = BackgroundColor(PRESSED_BUTTON_COLOR.into());
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(HOVER_BUTTON_COLOR.into());
            }
            Interaction::None => {
                *background_color = BackgroundColor(NORMAL_BUTTON_COLOR.into());
            }
        }
    }
}
