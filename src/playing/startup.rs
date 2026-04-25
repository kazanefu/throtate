use super::*;
use crate::course::{CourseID, SpawnCourseMessage};
use crate::course_selection::resources::SelectedCourseID;
use crate::hammer::definition::Pivot;
use crate::hammer::spawn_hammer;
use crate::state::GameState;
use bevy_rapier2d::prelude::*;
pub struct PlayingStartupPlugin;

impl Plugin for PlayingStartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            ((spawn_selected_course,).chain(), spawn_player),
        )
        .add_systems(
            Update,
            add_component_for_despawn.run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_selected_course(
    mut spawn_message: MessageWriter<SpawnCourseMessage>,
    id: Res<SelectedCourseID>,
) {
    spawn_message.write(SpawnCourseMessage(
        id.0.expect("you must have selected course"),
    ));
}
fn add_component_for_despawn(
    mut commands: Commands,
    course_entity_query: Query<Entity, (With<CourseID>, Without<DespawnOnExit<GameState>>)>,
    pivot_query: Query<Entity, (With<Pivot>, Without<DespawnOnExit<GameState>>)>,
) {
    for course_entity in &course_entity_query {
        commands
            .entity(course_entity)
            .insert(DespawnOnExit(GameState::Playing));
    }
    for pivot_entity in &pivot_query {
        commands
            .entity(pivot_entity)
            .insert(DespawnOnExit(GameState::Playing));
    }
}

fn spawn_player(mut commands: Commands) {
    let player_entity = spawn_hammer(&mut commands, Vec2 { x: 0.0, y: 0.0 })
        .insert(Player)
        .insert(DeathCount(0))
        .insert(TargetCheckPoint::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(DespawnOnExit(GameState::Playing))
        .id();
    commands.spawn(main_camera_bundle(player_entity));
}
