use bevy::prelude::*;
mod start_ui;
mod startup;
use crate::state::GameState;
pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), startup::spawn_start_camera)
            .add_plugins(start_ui::StartUiPlugin);
    }
}
