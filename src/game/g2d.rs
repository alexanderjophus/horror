use super::{despawn_screen, GameState};
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;

pub struct G2dPlugin;

impl Plugin for G2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(OnExit(GameState::Game), (despawn_screen::<OnGame2DScreen>,));
    }
}

#[derive(Component)]
struct OnGame2DScreen;

fn setup(mut commands: Commands) {
    // spawn 2D overlay
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            ..Default::default()
        },
        OnGame2DScreen,
    ));
}
