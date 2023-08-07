use bevy::gltf::{Gltf, GltfMesh};
use bevy::prelude::*;
use bevy::{audio::VolumeLevel, core_pipeline::clear_color::ClearColorConfig};
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

use super::{despawn_screen, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the game screen, it will focus on the state `GameState::Game`
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            AtmospherePlugin,
        ))
        .init_resource::<Insanity>()
        .init_resource::<Animations>()
        .add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(
            Update,
            (
                spawn_house,
                movement,
                camera_rotation,
                light_flicker,
                update_insanity,
            )
                .run_if(in_state(GameState::Game)),
        )
        // run if in game state and player_close_to_front_door
        .add_systems(
            Update,
            open_door.run_if(in_state(GameState::Game).and_then(player_close_to_front_door)),
        )
        .add_systems(
            OnExit(GameState::Game),
            (
                despawn_screen::<OnGame3DScreen>,
                despawn_screen::<OnGame2DScreen>,
            ),
        );
    }
}

#[derive(Resource, Default)]
struct Animations {
    open_door: Handle<AnimationClip>,
}

#[derive(Resource)]
struct Sounds {
    door_open: Handle<AudioSource>,
}

#[derive(Resource, Deref, DerefMut, Default)]
struct Insanity(u32);

#[derive(Resource)]
struct LoadingAssets(Vec<Handle<Gltf>>);

#[derive(Component)]
struct OnGame3DScreen;

#[derive(Component)]
struct OnGame2DScreen;

#[derive(Component)]
struct GameCamera;

#[derive(Component, Default)]
struct Player {
    flashlight_flicker: Timer,
}

#[derive(Component)]
struct KnockingWoodEmitter;

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load House
    let gltf = asset_server.load("models/house.glb");
    commands.insert_resource(LoadingAssets(vec![gltf.clone()]));

    commands.insert_resource(Sounds {
        door_open: asset_server.load("sounds/door_open.ogg"),
    });

    commands.spawn((
        SpatialAudioBundle {
            source: asset_server.load("sounds/knocking_wood.ogg"),
            settings: PlaybackSettings::LOOP,
            spatial: SpatialSettings::new(
                Transform::from_xyz(0.0, 1.6, -10.0),
                4.0,
                Vec3::new(0.0, 2.0, 5.0),
            ),
        },
        KnockingWoodEmitter,
    ));

    commands.insert_resource(AtmosphereModel::new(Nishita {
        sun_position: Vec3::new(0., 0., -1.),
        ..default()
    }));

    // spawn flashlight with camera
    commands
        .spawn((
            SpotLightBundle {
                transform: Transform::from_xyz(0.0, 1.4, -10.0).looking_at(Vec3::Y * 1.6, Vec3::Y),
                spot_light: SpotLight {
                    color: Color::rgb(0.8, 0.8, 0.8),
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
            RigidBody::Fixed,
            Collider::capsule_y(0.3, 0.2),
            KinematicCharacterController::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3dBundle::default(),
                AtmosphereCamera::default(),
                FogSettings {
                    color: Color::rgba(0.05, 0.05, 0.05, 1.0),
                    falloff: FogFalloff::Exponential { density: 0.15 },
                    ..Default::default()
                },
            ));
        });

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Insanity: ",
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 60.0,
                    color: Color::RED,
                    ..Default::default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..Default::default()
        }),
    );

    // spawn 2D overlay
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        camera: Camera {
            order: 1,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_house(
    mut commands: Commands,
    mut assets: ResMut<LoadingAssets>,
    assets_mesh: Res<Assets<Mesh>>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    assets.0.retain(|asset| {
        if let Some(gltf) = assets_gltf.get(asset) {
            let boarding = assets_gltfmesh.get(&gltf.named_meshes["boarding"]).unwrap();
            let mesh = &boarding.primitives[0].mesh.clone();
            commands.spawn((
                SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
                    ..Default::default()
                },
                RigidBody::Fixed,
                Collider::from_bevy_mesh(
                    assets_mesh.get(mesh).unwrap(),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap(),
            ));

            commands.insert_resource(Animations {
                open_door: gltf.animations[0].clone(),
            });
            return false;
        }
        true
    })
}

fn movement(
    time: Res<Time>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<&mut KinematicCharacterController>,
    transform_query: Query<&Transform, With<Player>>,
    mut knocking_wood_emitter: Query<
        &mut SpatialAudioSink,
        (
            With<KnockingWoodEmitter>,
            Without<KinematicCharacterController>,
        ),
    >,
) {
    for mut controller in query.iter_mut() {
        let transform = transform_query.single();
        for gamepad in gamepads.iter() {
            let mut translation = Vec3::ZERO;
            let left_stick_x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            if left_stick_x.abs() > 0.01 {
                let forward = transform.left() * Vec3::new(1.0, 0.0, 1.0);
                translation += forward * -left_stick_x * time.delta_seconds() * 3.0;
            }
            let left_stick_y = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
                .unwrap();
            if left_stick_y.abs() > 0.01 {
                let left = transform.forward() * Vec3::new(1.0, 0.0, 1.0);
                translation += left * left_stick_y * time.delta_seconds() * 3.0;
            }
            controller.translation = Some(translation);
            for emitter_transform in knocking_wood_emitter.iter_mut() {
                emitter_transform.set_listener_position(*transform, 4.0);
            }
        }
    }
}

fn camera_rotation(
    time: Res<Time>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    for gamepad in gamepads.iter() {
        let right_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
            .unwrap();
        let right_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
            .unwrap();

        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        pitch += right_stick_y * time.delta_seconds() * 2.0;
        pitch = pitch.clamp(-PI / 8.0, PI / 8.0);
        yaw -= right_stick_x * time.delta_seconds() * 2.0;
        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
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

fn update_insanity(insanity: Res<Insanity>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = insanity.0.to_string();
}

fn open_door(
    mut commands: Commands,
    animations: Res<Animations>,
    sounds: Res<Sounds>,
    mut insanity: ResMut<Insanity>,
    mut anim_player: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in anim_player.iter_mut() {
        insanity.0 += 1;
        commands.spawn(AudioBundle {
            source: sounds.door_open.clone(),
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.5)),
                ..Default::default()
            },
        });
        player.play(animations.open_door.clone());
    }
}

fn player_close_to_front_door(player_query: Query<&Transform, With<Player>>) -> bool {
    let player_transform = player_query.single();
    if player_transform
        .translation
        .distance_squared(Vec3::new(0.0, 0.0, 4.0))
        < 50.0
    {
        return true;
    }
    false
}
