use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct Debug3DPlugin;

impl Plugin for Debug3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
}
