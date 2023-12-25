use crate::events::PlayerDies;
use crate::resources::AppState;
use bevy::prelude::*;

pub fn game_over_when_player_dies(
    mut state: ResMut<NextState<AppState>>,
    mut ev_player_dies: EventReader<PlayerDies>,
) {
    for event in ev_player_dies.read() {
        state.set(AppState::GameOver);
    }
}


#[derive(Component)]
pub struct GameOver;

pub fn setup_game_over_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "Game Over | Press <space> to restart",
            TextStyle {
                font_size: 40.,
                color: Color::BLACK,
                ..default()
            },
        )])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            }), GameOver
    ));
}

pub fn handle_game_over(
    mut state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(AppState::MainMenu);
    }
}

pub fn cleanup_game_over_ui(mut commands: Commands, mut q: Query<Entity, With<GameOver>>) {
    for entity in q.iter_mut() {
        commands.entity(entity).despawn();
    }
}