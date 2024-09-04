use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use crate::{despawn_screen, GameState};

pub struct VHSPlugin;

impl Plugin for VHSPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<VHSShader>::default())
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnShader>);
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct VHSShader {
    #[uniform(100)]
    pub color: LinearRgba,

    #[texture(101, dimension = "2d")]
    #[sampler(102)]
    pub img: Handle<Image>,
}

impl Material for VHSShader {
    fn fragment_shader() -> ShaderRef {
        "shaders/vhs.wgsl".into()
    }
}

#[derive(Component)]
struct OnShader;

fn setup(mut commands: Commands) {
    // spawn 2D overlay
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::None,
                order: 2,
                ..Default::default()
            },
            ..Default::default()
        },
        OnShader,
    ));
    info!("Spawned Camera");
}
