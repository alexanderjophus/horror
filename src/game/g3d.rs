use super::{despawn_screen, GameState, Insanity, Player};
use bevy::gltf::{Gltf, GltfMesh};
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
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
        .add_systems(OnEnter(GameState::Game), setup)
        .add_systems(
            Update,
            (camera_rotation, light_flicker, spawn_house).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            movement.run_if(in_state(GameState::Game).and_then(first_audio_finished)),
        )
        .add_systems(
            Update,
            open_door.run_if(in_state(GameState::Game).and_then(player_close_to_front_door)),
        )
        .add_systems(OnExit(GameState::Game), (despawn_screen::<OnGame3DScreen>,));
    }
}

const PLAYER_INIT_LOCATION: Vec3 = Vec3::new(0.0, 1.4, -10.0);

#[derive(Component)]
struct OnGame3DScreen;

#[derive(Resource)]
struct LoadingAssets(Vec<Handle<Gltf>>);

#[derive(Resource, Default)]
struct Animations {
    open_door: Handle<AnimationClip>,
}

#[derive(Resource)]
struct Sounds {
    door_open: Handle<AudioSource>,
    knocking_wood: Handle<AudioSource>,
}

#[derive(Component)]
struct KnockingWoodEmitter;

#[derive(Component)]
struct Intro;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
    Look,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("models/house.glb");
    commands.insert_resource(LoadingAssets(vec![gltf.clone()]));

    commands.insert_resource(Sounds {
        door_open: asset_server.load("sounds/door_open.ogg"),
        knocking_wood: asset_server.load("sounds/knocking_wood.ogg"),
    });

    commands.insert_resource(AtmosphereModel::new(Nishita {
        sun_position: Vec3::new(0., 0., -1.),
        ..default()
    }));

    // commands.spawn((
    //     AudioBundle {
    //         source: asset_server.load("sounds/haunting_piano.ogg"),
    //         settings: PlaybackSettings::DESPAWN,
    //     },
    //     Intro,
    // ));

    // spawn flashlight with camera
    commands
        .spawn((
            SpotLightBundle {
                transform: Transform::from_translation(PLAYER_INIT_LOCATION)
                    .looking_at(Vec3::Y * PLAYER_INIT_LOCATION.y, Vec3::Y),
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
                input_map: InputMap::new([
                    (DualAxis::left_stick(), Action::Move),
                    (DualAxis::right_stick(), Action::Look),
                ]),
            },
            OnGame3DScreen,
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
}

fn spawn_house(
    mut commands: Commands,
    sounds: Res<Sounds>,
    mut assets: ResMut<LoadingAssets>,
    assets_mesh: Res<Assets<Mesh>>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    assets.0.retain(|asset| {
        if let Some(gltf) = assets_gltf.get(asset) {
            commands.spawn((
                SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
                    ..Default::default()
                },
                OnGame3DScreen,
            ));

            // spawn boarding colliders
            let boarding = assets_gltfmesh.get(&gltf.named_meshes["boarding"]).unwrap();
            let boarding_mesh = &boarding.primitives[0].mesh.clone();
            commands.spawn((
                RigidBody::Fixed,
                Collider::from_bevy_mesh(
                    assets_mesh.get(boarding_mesh).unwrap(),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap(),
                OnGame3DScreen,
            ));

            // spawn stairs colliders
            let stairs: &GltfMesh = assets_gltfmesh.get(&gltf.named_meshes["Stairs"]).unwrap();
            let stairs_mesh = &stairs.primitives[0].mesh.clone();
            commands.spawn((
                RigidBody::Fixed,
                Collider::from_bevy_mesh(
                    assets_mesh.get(stairs_mesh).unwrap(),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap(),
                OnGame3DScreen,
            ));

            commands.insert_resource(Animations {
                open_door: gltf.animations[0].clone(),
            });

            commands.spawn((
                SpatialAudioBundle {
                    source: sounds.knocking_wood.clone(),
                    settings: PlaybackSettings::LOOP,
                    spatial: SpatialSettings::new(
                        Transform::from_translation(PLAYER_INIT_LOCATION),
                        4.0,
                        Vec3::new(0.0, 2.0, 10.0),
                    ),
                },
                KnockingWoodEmitter,
                OnGame3DScreen,
            ));
            return false;
        }
        true
    })
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
    mut knocking_wood_emitter: Query<
        &mut SpatialAudioSink,
        (
            With<KnockingWoodEmitter>,
            Without<KinematicCharacterController>,
        ),
    >,
) {
    for (mut controller, transform, action_state) in query.iter_mut() {
        if action_state.pressed(Action::Move) {
            let mut translation = Vec3::ZERO;
            let axis_pair = action_state.clamped_axis_pair(Action::Move).unwrap();
            let forward = transform.left() * Vec3::new(1.0, 0.0, 1.0);
            let left = transform.forward() * Vec3::new(1.0, 0.0, 1.0);
            translation += forward * -axis_pair.x() * time.delta_seconds() * 3.0;
            translation += left * axis_pair.y() * time.delta_seconds() * 3.0;

            controller.translation = Some(translation);
            for emitter_transform in knocking_wood_emitter.iter_mut() {
                emitter_transform.set_listener_position(*transform, 4.0);
            }
        }
    }
}

fn camera_rotation(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ActionState<Action>), With<Player>>,
) {
    for (mut transform, action_state) in query.iter_mut() {
        if action_state.pressed(Action::Look) {
            let axis_pair = action_state.clamped_axis_pair(Action::Look).unwrap();
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

            pitch += axis_pair.y() * time.delta_seconds() * 2.0;
            pitch = pitch.clamp(-PI / 8.0, PI / 8.0);
            yaw -= axis_pair.x() * time.delta_seconds() * 2.0;
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

fn open_door(
    mut commands: Commands,
    animations: Res<Animations>,
    sounds: Res<Sounds>,
    mut insanity: ResMut<Insanity>,
    mut anim_player: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in anim_player.iter_mut() {
        insanity.0 += 1;
        // not sure this sound even plays
        commands.spawn((
            SpatialAudioBundle {
                source: sounds.door_open.clone(),
                settings: PlaybackSettings::ONCE,
                spatial: SpatialSettings::new(
                    Transform::from_translation(PLAYER_INIT_LOCATION),
                    4.0,
                    Vec3::new(2.7, 0.0, 0.0),
                ),
            },
            OnGame3DScreen,
        ));
        player.play(animations.open_door.clone());
    }
}

fn player_close_to_front_door(player_query: Query<&Transform, With<Player>>) -> bool {
    let player_transform = player_query.single();
    if player_transform
        .translation
        .distance_squared(Vec3::new(2.7, 0.0, 0.0))
        < 50.0
    {
        return true;
    }
    false
}

fn first_audio_finished(query: Query<&Intro>) -> bool {
    for _ in query.iter() {
        return false;
    }
    true
}
