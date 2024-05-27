use super::{despawn_screen, GameState};
use bevy::prelude::*;
use bevy_quickmenu::{
    style::Stylesheet, ActionTrait, Menu, MenuItem, MenuState, QuickMenuPlugin, ScreenTrait,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuEvent>()
            .add_plugins(QuickMenuPlugin::<Screens>::new())
            .add_systems(OnEnter(GameState::Menu), setup)
            .add_systems(Update, event_reader.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>);
    }
}

#[derive(Component)]
struct OnMenuScreen;

#[derive(Debug, Event)]
enum MenuEvent {
    Start,
}

#[derive(Debug, Clone, Default)]
struct MenuStateSettings {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum MenuActions {
    StartGame,
}

impl ActionTrait for MenuActions {
    type State = MenuStateSettings;
    type Event = MenuEvent;
    fn handle(&self, _: &mut MenuStateSettings, event_write: &mut EventWriter<MenuEvent>) {
        match self {
            MenuActions::StartGame => {
                event_write.send(MenuEvent::Start);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Screens {
    Root,
}

impl ScreenTrait for Screens {
    type Action = MenuActions;
    type State = MenuStateSettings;
    fn resolve(&self, state: &MenuStateSettings) -> Menu<Screens> {
        match self {
            Screens::Root => root_menu(state),
        }
    }
}

fn root_menu(_state: &MenuStateSettings) -> Menu<Screens> {
    Menu::new(
        "root",
        vec![MenuItem::action("start", MenuActions::StartGame)],
    )
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
        OnMenuScreen,
    ));

    let sheet = Stylesheet::default().with_background(BackgroundColor(Color::BISQUE));

    commands.insert_resource(MenuState::new(
        MenuStateSettings::default(),
        Screens::Root,
        Some(sheet),
    ))
}

fn event_reader(
    mut commands: Commands,
    mut event_reader: EventReader<MenuEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in event_reader.read() {
        match event {
            MenuEvent::Start => {
                next_state.set(GameState::Game);
                bevy_quickmenu::cleanup(&mut commands);
            }
        }
    }
}
