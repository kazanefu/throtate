use bevy::prelude::*;
use crate::state::GameState;
mod startup;
use startup::*;
pub struct CourseSelectionPlugin;

impl Plugin for CourseSelectionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameState::CourseSelection),(spawn_course_selection_cammera,course_0));
    }
}