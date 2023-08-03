use std::f32::consts::PI;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use rand::Rng;

use super::{despawn_screen, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the game screen, it will focus on the state `GameState::Game`
        app.init_resource::<Insanity>()
            .add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(
                Update,
                (movement, camera_rotation, light_flicker, update_insanity)
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

#[derive(Resource)]
struct Animations {
    open_door: Handle<AnimationClip>,
}

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

#[derive(Resource, Deref, DerefMut, Default)]
struct Insanity(u32);

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn House
    commands.spawn((SceneBundle {
        scene: asset_server.load("models/house.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, -10.0).looking_at(Vec3::NEG_Z, Vec3::Y),
        ..default()
    },));

    commands.insert_resource(Animations {
        open_door: asset_server.load("models/house.glb#Animation0"),
    });

    // spawn light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 800.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            color: Color::rgb(0.8, 0.8, 0.8),
            illuminance: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // spawn spotlights
    let light_locations = [(9.5, 0.0, -7.8), (-9.5, 0.0, -7.8)];
    for light_locations in light_locations {
        commands.spawn(SpotLightBundle {
            transform: Transform::from_xyz(light_locations.0, light_locations.1, light_locations.2)
                .looking_to(Vec3::Y, Vec3::Z),
            spot_light: SpotLight {
                color: Color::RED,
                intensity: 1600.0,
                range: 100.0,
                ..Default::default()
            },
            ..Default::default()
        });
    }
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(0.0, 5.4, -10.0).with_rotation(
            Quat::from_rotation_z(std::f32::consts::PI)
                .mul_quat(Quat::from_rotation_x(std::f32::consts::PI / 2.0)),
        ),
        spot_light: SpotLight {
            color: Color::TURQUOISE,
            intensity: 1600.0,
            range: 100.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // spawn flashlight with camera
    commands
        .spawn((
            SpotLightBundle {
                transform: Transform::from_xyz(0.0, 1.6, -20.0).looking_at(Vec3::Y * 1.6, Vec3::Y),
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
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3dBundle::default(),
                FogSettings {
                    color: Color::WHITE,
                    falloff: FogFalloff::Exponential { density: 1e-3 },
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
            ..Default::default()
        },
        camera: Camera {
            order: 1,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn movement(
    time: Res<Time>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    for gamepad in gamepads.iter() {
        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            let forward = transform.left() * Vec3::new(1.0, 0.0, 1.0);
            // should be direction use is facing
            transform.translation += forward * -left_stick_x * time.delta_seconds() * 3.0;
        }
        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.01 {
            let left = transform.forward() * Vec3::new(1.0, 0.0, 1.0);
            transform.translation += left * left_stick_y * time.delta_seconds() * 3.0;
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
    animations: Res<Animations>,
    mut insanity: ResMut<Insanity>,
    mut anim_player: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in anim_player.iter_mut() {
        insanity.0 += 1;
        player.play(animations.open_door.clone());
    }
}

fn player_close_to_front_door(player_query: Query<&Transform, With<Player>>) -> bool {
    let player_transform = player_query.single();
    if player_transform
        .translation
        .distance_squared(Vec3::new(0.0, 0.0, 0.0))
        < 200.0
    {
        return true;
    }
    false
}
