#[cfg(feature = "debug")]
mod debug3d;
mod g2d;
mod g3d;
mod vhs;

use super::{despawn_screen, GameState};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/door_open.ogg")]
    door_open: Handle<AudioSource>,
    #[asset(path = "audio/knocking_wood.ogg")]
    knocking_wood: Handle<AudioSource>,
    #[asset(path = "audio/haunting_piano.ogg")]
    intro: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct GltfAssets {
    #[asset(path = "models/house.glb")]
    house: Handle<Gltf>,
}

#[derive(Component, Default)]
pub(super) struct Player {
    flashlight_flicker: Timer,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            g2d::G2dPlugin,
            g3d::G3dPlugin,
            vhs::VHSPlugin,
            #[cfg(feature = "debug")]
            debug3d::Debug3DPlugin,
        ));
    }
}
