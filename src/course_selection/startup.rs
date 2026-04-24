use crate::state::GameState;
use bevy::prelude::*;

fn course_selection_cammera_bundle() -> impl Bundle {
    (Camera2d, DespawnOnExit(GameState::Start))
}

pub fn spawn_course_selection_cammera(mut commands: Commands) {
    commands.spawn(course_selection_cammera_bundle());
}

use crate::course::SpawnCourseMessage;

pub fn course_0(mut spawn_course_writer: MessageWriter<SpawnCourseMessage>) {
    spawn_course_writer.write(SpawnCourseMessage(0));
}
