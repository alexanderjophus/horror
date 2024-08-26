use crate::{GameState, GAME_NAME};

use super::{despawn_screen, GameplayState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_systems(
            Update,
            ui.run_if(in_state(GameState::Game).and_then(in_state(GameplayState::Paused))),
        )
        .add_systems(
            OnExit(GameplayState::Paused),
            despawn_screen::<OnPauseScreen>,
        );
    }
}

#[derive(Component)]
struct OnPauseScreen;

fn ui(
    mut contexts: EguiContexts,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_gameplay_state: ResMut<NextState<GameplayState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 10.0);

            ui.add(egui::Label::new(egui::RichText::new(GAME_NAME).size(64.0)));

            ui.add_space(10.0);

            let resume = ui.add(egui::Button::new(egui::RichText::new("Resume").size(32.0)));
            let main_menu = ui.add(egui::Button::new(
                egui::RichText::new("Main Menu").size(24.0),
            ));

            if resume.clicked() {
                next_gameplay_state.set(GameplayState::Playing);
            }
            if main_menu.clicked() {
                next_game_state.set(GameState::Menu);
            }
        });
    });
}
