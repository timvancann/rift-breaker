use bevy::prelude::*;

use crate::player::PlayerHealthChanged;

#[derive(Component)]
pub struct PlayerHealthUIPlugin;

impl Plugin for PlayerHealthUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_event::<PlayerHealthChanged>()
            .add_systems(Update, update_ui);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((TextBundle::from_sections([
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
    }),));
}

fn update_ui(
    mut ev_player_health: EventReader<PlayerHealthChanged>,
    mut q_text: Query<&mut Text, With<Text>>,
) {
    for mut ev in ev_player_health.read() {
        for mut text in q_text.iter_mut() {
            text.sections[1].value = ev.current.to_string();
            text.sections[3].value = ev.max.to_string();
        }
    }
}
