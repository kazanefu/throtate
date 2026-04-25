use crate::state::GameState;
use bevy::prelude::*;

fn start_camera_bundle() -> impl Bundle {
    (Camera2d, DespawnOnExit(GameState::Start))
}

pub fn spawn_start_camera(mut commands: Commands) {
    commands.spawn(start_camera_bundle());

}
