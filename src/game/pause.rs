use crate::GAME_NAME;

use super::{despawn_screen, GameplayState};
use bevy::prelude::*;
use bevy_quickmenu::{
    style::Stylesheet, ActionTrait, Menu, MenuItem, MenuState, QuickMenuPlugin, ScreenTrait,
};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuEvent>()
            .add_plugins(QuickMenuPlugin::<Screens>::new())
            .add_systems(OnEnter(GameplayState::Paused), setup)
            .add_systems(
                OnExit(GameplayState::Paused),
                (despawn_screen::<OnPauseScreen>, close_menu),
            );
    }
}

#[derive(Component)]
struct OnPauseScreen;

#[derive(Debug, Event)]
enum MenuEvent {}

#[derive(Debug, Clone, Default)]
struct PauseState {
    sound_on: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum MenuActions {
    ToggleSound,
}

impl ActionTrait for MenuActions {
    type State = PauseState;
    type Event = MenuEvent;
    fn handle(&self, state: &mut PauseState, _: &mut EventWriter<MenuEvent>) {
        match self {
            MenuActions::ToggleSound => {
                state.sound_on = !state.sound_on;
            }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 4,
                ..Default::default()
            },
            ..Default::default()
        },
        OnPauseScreen,
    ));

    let sheet = Stylesheet::default();

    commands.insert_resource(MenuState::new(
        PauseState::default(),
        Screens::Root,
        Some(sheet),
    ))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Screens {
    Root,
    Sound,
}

impl ScreenTrait for Screens {
    type Action = MenuActions;
    type State = PauseState;
    fn resolve(&self, state: &PauseState) -> Menu<Screens> {
        match self {
            Screens::Root => root_menu(state),
            Screens::Sound => sound_menu(state),
        }
    }
}

fn root_menu(_state: &PauseState) -> Menu<Screens> {
    Menu::new(
        "root",
        vec![
            MenuItem::headline(GAME_NAME),
            MenuItem::screen("Sound (TODO)", Screens::Sound),
        ],
    )
}

fn sound_menu(state: &PauseState) -> Menu<Screens> {
    Menu::new(
        "sound",
        vec![
            MenuItem::headline("Sound"),
            MenuItem::action(
                if state.sound_on {
                    "Sound: On"
                } else {
                    "Sound: Off"
                },
                MenuActions::ToggleSound,
            ),
        ],
    )
}

fn close_menu(mut commands: Commands) {
    bevy_quickmenu::cleanup(&mut commands);
}
