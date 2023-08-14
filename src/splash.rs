use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::{despawn_screen, GameState, GAME_NAME};
use crate::game::{AudioAssets, GltfAssets};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app.add_loading_state(
            LoadingState::new(GameState::Splash).continue_to_state(GameState::Game),
        )
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Splash)
        .add_collection_to_loading_state::<_, GltfAssets>(GameState::Splash)
        // When entering the state, spawn everything needed for this screen
        // .add_system(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
        .add_systems(OnEnter(GameState::Splash), splash_setup)
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(
            OnExit(GameState::Splash),
            (
                despawn_screen::<OnSplashScreen>,
                despawn_screen::<SplashCamera>,
            ),
        );
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

#[derive(Component)]
struct SplashCamera;

fn splash_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), SplashCamera));

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                GAME_NAME,
                TextStyle {
                    font_size: 180.0,
                    color: Color::DARK_GRAY,
                    ..Default::default()
                },
            ),
            ..Default::default()
        },
        OnSplashScreen,
    ));
}
