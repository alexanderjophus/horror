use super::{despawn_screen, GameState};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, ui.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>);
    }
}

#[derive(Component)]
struct OnMenuScreen;

fn ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut writer: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 10.0);

            ui.add(egui::Label::new(
                egui::RichText::new("Jophus' Horror").size(64.0),
            ));

            ui.add_space(10.0);

            let play = ui.add(egui::Button::new(egui::RichText::new("Play").size(32.0)));
            let quit = ui.add(egui::Button::new(egui::RichText::new("Quit").size(24.0)));

            if play.clicked() {
                next_state.set(GameState::Game);
            }

            if quit.clicked() {
                writer.send(AppExit::Success);
            }
        })
    });
}
