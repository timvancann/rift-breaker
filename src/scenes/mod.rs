mod game_over;
mod main_menu;

use crate::resources::AppState;
use crate::scenes::main_menu::*;
use bevy::prelude::*;
use crate::events::PlayerDies;
use crate::scenes::game_over::{cleanup_game_over_ui, game_over_when_player_dies, handle_game_over, setup_game_over_ui};

#[derive(Component)]
pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_systems(Startup, setup_main_menu)
            .add_event::<PlayerDies>()
            //mainmenu
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                handle_main_menu.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), hide_main_menu)
            //ingame
            .add_systems(Update, game_over_when_player_dies.run_if(in_state(AppState::InGame)))
            //gameover
            .add_systems(OnEnter(AppState::GameOver), setup_game_over_ui)
            .add_systems(Update, handle_game_over.run_if(in_state(AppState::GameOver)))
            .add_systems(OnExit(AppState::GameOver), cleanup_game_over_ui)
        ;
    }
}
