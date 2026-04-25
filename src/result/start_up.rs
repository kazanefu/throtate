use crate::state::GameState;
use bevy::prelude::*;

fn result_camera_bundle() -> impl Bundle {
    (Camera2d, DespawnOnExit(GameState::Result))
}

pub fn spawn_result_camera(mut commands: Commands) {
    commands.spawn(result_camera_bundle());
}