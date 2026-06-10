use bevy::prelude::*;

use crate::state::GameState;

mod ui;

pub struct HintPlugin;

impl Plugin for HintPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), ui::spawn_hint_ui)
            .add_systems(Update, ui::hint_ui_show);
    }
}
