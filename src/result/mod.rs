use bevy::prelude::*;

mod start_up;
mod ui;

use crate::state::GameState;

pub struct ResultPlugin;

impl Plugin for ResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ui::ResultUiPlugin)
            .add_systems(OnEnter(GameState::Result), start_up::spawn_result_camera);
    }
}
