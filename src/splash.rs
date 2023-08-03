use bevy::prelude::*;

use super::{despawn_screen, GameState, GAME_NAME};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            // .add_system(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            // While in this state, run the `countdown` system
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(
                OnExit(GameState::Splash),
                (
                    despawn_screen::<OnSplashScreen>,
                    despawn_screen::<SplashCamera>,
                ),
            );
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Component)]
struct SplashCamera;

fn splash_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), SplashCamera));

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                GAME_NAME,
                TextStyle {
                    font_size: 180.0,
                    color: Color::DARK_GRAY,
                    ..Default::default()
                },
            ),
            ..Default::default()
        },
        OnSplashScreen,
    ));

    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(3., TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state_next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state_next_state.set(GameState::Game);
    }
}
