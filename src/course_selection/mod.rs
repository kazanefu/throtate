use crate::state::GameState;
use bevy::prelude::*;
mod selection_ui;
mod startup;
mod resources;
use startup::*;
pub struct CourseSelectionPlugin;

impl Plugin for CourseSelectionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(selection_ui::SelectionUiPlugin)
            .insert_resource(resources::SelectedCourseID(None))
            .add_systems(
                OnEnter(GameState::CourseSelection),
                (spawn_course_selection_cammera, course_0),
            );
    }
}
