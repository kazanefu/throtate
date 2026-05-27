use crate::{config::GameConfig, state::GameState};
use bevy::prelude::*;

pub fn exit(
    mut game_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    config: Res<GameConfig>,
) {
    if config.input.exit.just_pressed(&keys) {
        game_state.set(GameState::Loading);
    }
}
