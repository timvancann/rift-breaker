use bevy::prelude::*;

use crate::events::PlayerHealthChanged;
use crate::resources::{AppState, XP};

#[derive(Component)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (setup_player_health, setup_score_board, setup_xp_ui),
        )
        .add_event::<PlayerHealthChanged>()
        .add_systems(
            Update,
            (update_player_health_ui, update_score_ui, update_xp_ui)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Resource, Default)]
pub struct Score(pub i32);

#[derive(Component)]
struct PlayerHealthUI;

#[derive(Component)]
struct ScoreUI;

#[derive(Component)]
struct XpUI;

fn setup_player_health(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Health: ",
                TextStyle {
                    font_size: 40.,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 40.,
                color: Color::ORANGE_RED,
                ..default()
            }),
            TextSection::new(
                " / ",
                TextStyle {
                    font_size: 40.,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 40.,
                color: Color::ORANGE_RED,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        PlayerHealthUI,
    ));
}

fn setup_score_board(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score ",
                TextStyle {
                    font_size: 40.,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 40.,
                color: Color::DARK_GREEN,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        ScoreUI,
    ));
}

fn setup_xp_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "XP ",
                TextStyle {
                    font_size: 40.,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 40.,
                color: Color::YELLOW,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        XpUI,
    ));
}

fn update_player_health_ui(
    mut ev_player_health: EventReader<PlayerHealthChanged>,
    mut q_text: Query<&mut Text, With<PlayerHealthUI>>,
) {
    for ev in ev_player_health.read() {
        for mut text in q_text.iter_mut() {
            text.sections[1].value = ev.current.to_string();
            text.sections[3].value = ev.max.to_string();
        }
    }
}

fn update_score_ui(score: Res<Score>, mut q_text: Query<&mut Text, With<ScoreUI>>) {
    for mut text in q_text.iter_mut() {
        text.sections[1].value = score.0.to_string();
    }
}

fn update_xp_ui(mut q_text: Query<&mut Text, With<XpUI>>, xp: Res<XP>) {
    for mut text in q_text.iter_mut() {
        text.sections[1].value = xp.0.to_string();
    }
}
