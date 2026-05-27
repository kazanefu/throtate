use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SelectedCourseID(pub Option<usize>);
