use std::time::Instant;

use super::{despawn_screen, GameState};
use bevy::prelude::*;

#[derive(Component, Default)]
pub(super) struct Vhs {
    play_flash: Timer,
}

#[derive(Component, Default)]
pub(super) struct Timestamp {
    ts: Time<Real>,
}

pub struct G2dPlugin;

impl Plugin for G2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(Update, update_vhs_play.run_if(in_state(GameState::Game)))
            .add_systems(Update, update_vhs_timer.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), (despawn_screen::<OnGame2DScreen>,));
    }
}

#[derive(Component)]
struct OnGame2DScreen;

fn setup(mut commands: Commands) {
    // timestamp
    commands.spawn((
        TextBundle::from_section(
            "00:00:00",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..Default::default()
        }),
        Timestamp {
            ts: Time::new(Instant::now()),
        },
        OnGame2DScreen,
    ));

    // play button
    commands.spawn((
        TextBundle::from_section(
            "Play: \u{25B6}",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..Default::default()
        }),
        Vhs {
            play_flash: Timer::from_seconds(0.5, TimerMode::Repeating),
        },
        OnGame2DScreen,
    ));

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

fn update_vhs_play(time: Res<Time<Real>>, mut query: Query<(&mut Vhs, &mut Text)>) {
    let (mut vhs, mut text) = query.single_mut();
    // toggle visibility on repeat
    if vhs.play_flash.tick(time.delta()).just_finished() {
        text.sections[0].style.color = if text.sections[0].style.color == Color::WHITE {
            Color::NONE
        } else {
            Color::WHITE
        };
    }
}

fn update_vhs_timer(mut query: Query<(&mut Timestamp, &mut Text)>) {
    let (mut ts, mut text) = query.single_mut();
    ts.ts.update();
    text.sections[0].value = format!(
        "{:02}:{:02}:{:02}",
        ts.ts.elapsed_seconds() as u32 / 3600,
        ts.ts.elapsed_seconds() as u32 / 60 % 60,
        ts.ts.elapsed_seconds() as u32 % 60
    );
}
