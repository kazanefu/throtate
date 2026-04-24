use bevy::prelude::*;
use crate::state::GameState;
use crate::course::{SpawnCourseMessage,CourseID};
use crate::course_selection::resources::SelectedCourseID;
use crate::hammer::spawn_hammer;
use super::*;
pub struct PlayingStartupPlugin;

impl Plugin for PlayingStartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), ((spawn_selected_course,add_component_for_despawn).chain(),spawn_player));
    }
}

fn spawn_selected_course(mut spawn_message: MessageWriter<SpawnCourseMessage>, id: Res<SelectedCourseID>) {
    spawn_message.write(SpawnCourseMessage(id.0.expect("you must have selected course")));
}
fn add_component_for_despawn(mut commands: Commands, course_entity_query: Query<Entity,With<CourseID>>) {
    for course_entity in &course_entity_query {
        commands.entity(course_entity).insert(DespawnOnExit(GameState::Playing));
    }
}

fn spawn_player(mut commands: Commands) {
    let player_entity = spawn_hammer(&mut commands, Vec2 { x: 0.0, y: 0.0 }).insert(Player).insert(DeathCount(0)).id();
    commands.spawn(main_camera_bundle(player_entity));
}