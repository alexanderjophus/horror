#[cfg(feature = "debug")]
mod debug3d;
mod g2d;
mod g3d;
mod pp;

use super::{despawn_screen, GameState};
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut, Default)]
pub(super) struct Insanity(u32);

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
                pp::BlurPlugin,
                #[cfg(feature = "debug")]
                debug3d::Debug3DPlugin,
            ))
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(Update, tick_insanity.run_if(in_state(GameState::Game)));
    }
}

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
