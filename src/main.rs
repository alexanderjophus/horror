#[cfg(feature = "debug")]
mod debug;
mod game;
mod splash;

use bevy::prelude::*;

pub const GAME_NAME: &str = "Horror Game";

// Enum that will be used as a global state for the game
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Game,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        // Declare the game state
        .add_state::<GameState>()
        // Adds the plugins for each state
        .add_plugins((
            splash::SplashPlugin,
            game::GamePlugin,
            #[cfg(feature = "debug")]
            debug::DebugPlugin,
        ))
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
