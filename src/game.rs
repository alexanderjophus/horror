mod g2d;
mod g3d;

use super::{despawn_screen, GameState};
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut, Default)]
pub(crate) struct Insanity(u32);

#[derive(Component, Default)]
pub(crate) struct Player {
    flashlight_flicker: Timer,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Insanity>()
            .add_plugins((g2d::G2dPlugin, g3d::G3dPlugin));
    }
}
