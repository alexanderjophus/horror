use bevy::prelude::*;
use rand::Rng;

use super::{despawn_screen, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the game screen, it will focus on the state `GameState::Game`
        app
            // When entering the state, spawn everything needed for this screen
            // .add_system(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_systems(OnEnter(GameState::Game), game_setup)
            // While in this state, run the `game` system
            .add_systems(
                Update,
                (movement, camera_movement, camera_flicker).run_if(in_state(GameState::Game)),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(
                OnExit(GameState::Game),
                (despawn_screen::<OnGameScreen>, despawn_screen::<GameCamera>),
            );
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct GameCamera;

#[derive(Component, Default)]
struct Player {
    flashlight_flicker: Timer,
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn House
    commands.spawn((SceneBundle {
        scene: asset_server.load("models/house.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, -10.0).looking_at(Vec3::NEG_Z, Vec3::Y),
        ..default()
    },));

    // spawn light
    // todo figure out lighting
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
    let camera_locations = [(9.5, 0.0, -7.8), (-9.5, 0.0, -7.8)];
    for camera_locations in camera_locations {
        commands.spawn(SpotLightBundle {
            transform: Transform::from_xyz(
                camera_locations.0,
                camera_locations.1,
                camera_locations.2,
            )
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

    // spawn camera
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
            let forward = transform.left();
            // should be direction use is facing
            transform.translation += forward * -left_stick_x * time.delta_seconds() * 4.0;
        }
        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.01 {
            let left = transform.forward();
            transform.translation += left * left_stick_y * time.delta_seconds() * 4.0;
        }
    }
}

fn camera_movement(
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
        if right_stick_x.abs() > 0.01 {
            transform.rotate(Quat::from_rotation_y(
                -right_stick_x * time.delta_seconds() * 2.0,
            ));
        }
    }
}

fn camera_flicker(time: Res<Time>, mut query: Query<(&mut Player, &mut SpotLight)>) {
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
