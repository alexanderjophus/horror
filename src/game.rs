mod blur;
#[cfg(feature = "debug")]
mod debug3d;
mod g2d;
mod g3d;

use super::{despawn_screen, GameState};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Resource, Deref, DerefMut, Default)]
pub(super) struct Insanity(u32);

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/door_open.ogg")]
    door_open: Handle<AudioSource>,
    #[asset(path = "audio/knocking_wood.ogg")]
    knocking_wood: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct GltfAssets {
    #[asset(path = "models/house.glb")]
    house: Handle<Gltf>,
}

#[derive(Component)]
struct InsanityTimer(Timer);

#[derive(Component, Default)]
pub(super) struct Player {
    flashlight_flicker: Timer,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Insanity>()
            .add_plugins((
                g2d::G2dPlugin,
                g3d::G3dPlugin,
                blur::BlurPlugin,
                #[cfg(feature = "debug")]
                debug3d::Debug3DPlugin,
            ))
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(Update, tick_insanity.run_if(in_state(GameState::Game)));
    }
}

// todo remove this once I figure out a mechanic to increase it more
fn setup(mut commands: Commands) {
    commands.spawn((InsanityTimer(Timer::from_seconds(
        10.0,
        TimerMode::Repeating,
    )),));
}

fn tick_insanity(
    time: Res<Time>,
    mut insanity: ResMut<Insanity>,
    mut insanity_query: Query<&mut InsanityTimer>,
) {
    let mut timer = insanity_query.single_mut();
    timer.0.tick(time.delta());
    if timer.0.finished() {
        insanity.0 += 1;
    }
}
