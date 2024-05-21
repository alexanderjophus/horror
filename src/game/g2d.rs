use super::{despawn_screen, GameState, Insanity};
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;

pub struct G2dPlugin;

impl Plugin for G2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(Update, update_insanity.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), (despawn_screen::<OnGame2DScreen>,));
    }
}

#[derive(Component)]
struct OnGame2DScreen;

fn setup(mut commands: Commands) {
    commands.spawn((
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
        OnGame2DScreen,
    ));

    // spawn 2D overlay
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 2,
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            ..Default::default()
        },
        OnGame2DScreen,
    ));
}

fn update_insanity(insanity: Res<Insanity>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = insanity.0.to_string();
}
