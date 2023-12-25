use crate::resources::AppState;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "Main Menu | Press <space> to start",
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
        }),
        MainMenu,
    ));
}

pub fn handle_main_menu(
    mut state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(AppState::InGame);
    }
}

pub fn hide_main_menu(mut commands: Commands, mut q: Query<Entity, With<MainMenu>>) {
    for entity in q.iter_mut() {
        commands.entity(entity).despawn();
    }
}
