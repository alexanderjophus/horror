mod game;
mod splash;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const GAME_NAME: &str = "Nightmare Manor";

// Enum that will be used as a global state for the game
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Game,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
        ))
        // Declare the game state
        .add_state::<GameState>()
        // Adds the plugins for each state
        .add_plugins((splash::SplashPlugin, game::GamePlugin))
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
