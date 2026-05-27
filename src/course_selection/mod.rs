use crate::state::GameState;
use bevy::prelude::*;
pub mod resources;
pub mod selection_ui;
mod startup;
use startup::*;
pub struct CourseSelectionPlugin;

impl Plugin for CourseSelectionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(selection_ui::SelectionUiPlugin)
            .init_resource::<resources::SelectedCourseID>()
            .add_systems(
                OnEnter(GameState::CourseSelection),
                (spawn_course_selection_cammera, course_0),
            );
    }
}
