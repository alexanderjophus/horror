use crate::GameState;

use super::{despawn_screen, AudioAssets, GameplayState, GltfAssets, Player, TextureAssets};
use bevy::asset::LoadState;
use bevy::core_pipeline::Skybox;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

pub struct G3dPlugin;

impl Plugin for G3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<Action>::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_systems(OnEnter(GameState::Game), (setup, spawn_house))
        .add_systems(
            Update,
            (camera_rotation, light_flicker)
                .run_if(in_state(GameState::Game))
                .run_if(in_state(GameplayState::Playing)),
        )
        // restrict player movement until the intro is finished
        .add_systems(
            Update,
            movement.run_if(
                in_state(GameState::Game)
                    .and_then(in_state(GameplayState::Playing))
                    .and_then(intro_finished),
            ),
        )
        .add_systems(OnExit(GameState::Game), (despawn_screen::<OnGame3DScreen>,));
    }
}

const PLAYER_INIT_LOCATION: Vec3 = Vec3::new(0.0, 0.0, 1000.0);

#[derive(Component)]
struct OnGame3DScreen;

#[derive(Component)]
struct Intro;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
    Look,
}

impl Actionlike for Action {
    // Record what kind of inputs make sense for each action.
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis,
            Self::Look => InputControlKind::DualAxis,
        }
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sounds: Res<AudioAssets>,
    textures: ResMut<TextureAssets>,
) {
    commands.spawn((
        AudioBundle {
            source: sounds.intro.clone(),
            settings: PlaybackSettings::DESPAWN,
        },
        Name::new("intro"),
        Intro,
        OnGame3DScreen,
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 320.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.0, 0.0)
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        },
        Name::new("moon"),
    ));

    if asset_server.load_state(&textures.skybox) == LoadState::Loaded {
        let image = images.get_mut(&textures.skybox).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }
    }

    // spawn ground
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(100.0, 0.1, 100.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(100.0, 0.1, 100.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.5, 0.5),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.1, 0.0),
            ..Default::default()
        },
        Name::new("ground"),
        OnGame3DScreen,
    ));

    // spawn box around world
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(20.0, 10.0, 1.0),
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 20.0)),
        Name::new("back wall"),
        OnGame3DScreen,
    ));
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(20.0, 10.0, 1.0),
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, -20.0)),
        Name::new("front wall"),
        OnGame3DScreen,
    ));
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1.0, 10.0, 20.0),
        TransformBundle::from(Transform::from_xyz(20.0, 0.0, 0.0)),
        Name::new("right wall"),
        OnGame3DScreen,
    ));
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1.0, 10.0, 20.0),
        TransformBundle::from(Transform::from_xyz(-20.0, 0.0, 0.0)),
        Name::new("left wall"),
        OnGame3DScreen,
    ));

    // spawn flashlight with camera
    commands
        .spawn((
            SpotLightBundle {
                transform: Transform::from_translation(PLAYER_INIT_LOCATION)
                    .looking_at(Vec3::Y * PLAYER_INIT_LOCATION.y, Vec3::Y),
                spot_light: SpotLight {
                    color: Color::srgb(0.8, 0.8, 0.8),
                    intensity: 200.0,
                    range: 100.0,
                    inner_angle: 0.0,
                    outer_angle: 0.35,
                    ..Default::default()
                },
                ..Default::default()
            },
            Player {
                flashlight_flicker: Timer::from_seconds(0.1, TimerMode::Once),
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
            Collider::capsule_y(0.5, 0.2),
            KinematicCharacterController {
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Absolute(0.5),
                    min_width: CharacterLength::Absolute(0.2),
                    include_dynamic_bodies: true,
                }),
                snap_to_ground: Some(CharacterLength::Absolute(0.5)),
                ..Default::default()
            },
            InputManagerBundle::<Action> {
                // Stores "which actions are currently pressed"
                action_state: ActionState::default(),
                // Describes how to convert from player inputs into those actions
                input_map: InputMap::default()
                    .with_dual_axis(Action::Move, GamepadStick::LEFT)
                    .with_dual_axis(Action::Look, GamepadStick::RIGHT),
            },
            Name::new("player"),
            OnGame3DScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 0.7, 0.0),
                    ..Default::default()
                },
                Skybox {
                    image: textures.skybox.clone(),
                    brightness: 100.0,
                },
                // AtmosphereCamera::default(),
                FogSettings {
                    color: Color::srgba(0.05, 0.05, 0.05, 1.0),
                    falloff: FogFalloff::Exponential { density: 0.15 },
                    ..Default::default()
                },
            ));
        });

    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(210, 220, 240),
        brightness: 1.0,
    });
}

fn spawn_house(mut commands: Commands, assets: Res<GltfAssets>, assets_gltf: Res<Assets<Gltf>>) {
    if let Some(gltf) = assets_gltf.get(&assets.house.clone()) {
        commands.spawn((
            SceneBundle {
                scene: gltf.scenes[0].clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
                ..Default::default()
            },
            AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh),
                ..Default::default()
            },
            OnGame3DScreen,
        ));

        commands.spawn((
            SpatialBundle::from_transform(Transform::from_translation(PLAYER_INIT_LOCATION)),
            SpatialListener::new(4.0),
        ));
    }
}

fn movement(
    time: Res<Time>,
    mut query: Query<
        (
            &mut KinematicCharacterController,
            &Transform,
            &ActionState<Action>,
        ),
        With<Player>,
    >,
) {
    for (mut controller, transform, action_state) in query.iter_mut() {
        if action_state.pressed(&Action::Move) {
            let mut translation = Vec3::ZERO;
            let axis_pair = action_state.clamped_axis_pair(&Action::Move);
            let forward = transform.left();
            let left = transform.forward();
            translation += forward * -axis_pair.x * time.delta_seconds() * 3.0;
            translation += left * axis_pair.y * time.delta_seconds() * 3.0;

            controller.translation = Some(translation);
        }
    }
}

fn camera_rotation(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ActionState<Action>), With<Player>>,
) {
    for (mut transform, action_state) in query.iter_mut() {
        if action_state.pressed(&Action::Look) {
            let axis_pair = action_state.clamped_axis_pair(&Action::Look);
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

            pitch += axis_pair.y * time.delta_seconds() * 2.0;
            pitch = pitch.clamp(-PI / 8.0, PI / 8.0);
            yaw -= axis_pair.x * time.delta_seconds() * 2.0;
            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}

fn light_flicker(time: Res<Time>, mut query: Query<(&mut Player, &mut SpotLight)>) {
    let (mut player, mut light) = query.single_mut();
    player.flashlight_flicker.tick(time.delta());
    if rand::thread_rng().gen_range(0..50) == 0 {
        light.intensity = 50.0;
        player.flashlight_flicker.reset();
    }
    if player.flashlight_flicker.finished() {
        light.intensity = 200.0;
    }
}

fn intro_finished(query: Query<&Intro>) -> bool {
    if query.iter().next().is_some() {
        return false;
    }
    true
}
