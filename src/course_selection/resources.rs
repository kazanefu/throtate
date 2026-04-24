use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectedCourseID(pub Option<usize>);