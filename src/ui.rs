use bevy::prelude::*;

#[derive(Resource, Default, Clone, Copy)]
pub struct PlayerHealth {
    pub current: f32,
}

#[derive(Component)]
pub struct PlayerHealthUIPlugin;

impl Plugin for PlayerHealthUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerHealth>()
            .add_systems(Startup, setup_ui)
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
    ])
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(5.0),
        left: Val::Px(5.0),
        ..default()
    }),));
}

fn update_ui(player_health: Res<PlayerHealth>, mut q: Query<&mut Text>) {
    for mut text in q.iter_mut() {
        text.sections[1].value = player_health.current.to_string();
    }
}
