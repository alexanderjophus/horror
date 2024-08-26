#[cfg(feature = "debug")]
mod debug3d;
mod g2d;
mod g3d;
// mod pause;
#[cfg(feature = "shaders")]
mod vhs;

use super::{despawn_screen, GameState};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameplayState {
    #[default]
    Playing,
    // Paused,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Pause,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/haunting_piano.ogg")]
    intro: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct GltfAssets {
    #[asset(path = "models/world.glb")]
    house: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/skybox.png")]
    skybox: Handle<Image>,
}

#[derive(Component, Default)]
pub(super) struct Player {
    flashlight_flicker: Timer,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<Action>::default(),
            g2d::G2dPlugin,
            g3d::G3dPlugin,
            #[cfg(feature = "shaders")]
            vhs::VHSPlugin,
            // pause::PausePlugin,
            #[cfg(feature = "debug")]
            debug3d::Debug3DPlugin,
        ))
        .init_state::<GameplayState>()
        .add_systems(OnEnter(GameState::Game), setup);
        // .add_systems(Update, toggle_pause.run_if(in_state(GameState::Game)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<Action> {
        // Stores "which actions are currently pressed"
        action_state: ActionState::default(),
        // Describes how to convert from player inputs into those actions
        input_map: InputMap::new([(Action::Pause, GamepadButtonType::Start)]),
    });
}

// fn toggle_pause(
//     state: Res<State<GameplayState>>,
//     mut next_state: ResMut<NextState<GameplayState>>,
//     query: Query<&ActionState<Action>>,
// ) {
//     let action_state = query.single();
//     if action_state.just_pressed(&Action::Pause) {
//         match state.get() {
//             GameplayState::Playing => next_state.set(GameplayState::Paused),
//             GameplayState::Paused => next_state.set(GameplayState::Playing),
//         }
//     }
// }
