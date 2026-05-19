use bevy::prelude::*;

use crate::JpFont;

use super::course_items::*;

use super::*;

#[derive(Message)]
pub struct SpawnCourseMessage(pub usize);

pub fn spawn_course_from_id(
    mut commands: Commands,
    course_list_res: Res<CourseListResource>,
    mut spawn_course_message: MessageReader<SpawnCourseMessage>,
    font: Res<JpFont>,
    config: Res<crate::config::GameConfig>,
) {
    for SpawnCourseMessage(id) in spawn_course_message.read() {
        let course = course_list_res
            .0
            .iter()
            .find(|(course_entry, _course)| course_entry.id == *id);
        match course {
            Some((_course_entry, course)) => {
                let course_entity = commands
                    .spawn((CourseID::new(*id), Transform::from_xyz(0.0, 0.0, 0.0)))
                    .id();
                for entity in &course.entities {
                    let item_entity = spawn_course_from_entities(
                        &mut commands,
                        entity,
                        font.get(),
                        &config,
                    )
                    .id();
                    commands.entity(course_entity).add_child(item_entity);
                }
            }
            None => warn!("failed to get course from course list loaded at the start of game"),
        }
    }
}

fn spawn_course_from_entities<'a>(
    commands: &'a mut Commands,
    entity: &EntityData,
    font: &Handle<Font>,
    config: &crate::config::GameConfig,
) -> EntityCommands<'a> {
    let (x, y) = (entity.x, entity.y);
    let box_size = config.course.one_box_size;
    match &entity.kind {
        EntityKind::Ground { width, height } => {
            commands.spawn(ground::ground_bundle(x, y, *width, *height))
        }
        EntityKind::Checkpoint { priority } => {
            commands.spawn(checkpoint::check_point_bundle(x, y, *priority, box_size))
        }
        EntityKind::Breakable { required_speed } => {
            commands.spawn(breakable_box::breakable_box_bundle(
                x,
                y,
                *required_speed,
                box_size,
            ))
        }
        EntityKind::Death => commands.spawn(death_box::death_box_bundle(x, y, box_size)),
        EntityKind::Turret { interval, rotation, bullet_lifetime } => {
            turret::spawn_turret(commands, x, y, *interval, *rotation, *bullet_lifetime, box_size)
        }
        EntityKind::Goal => commands.spawn(goal::goal_bundle(x, y, box_size)),
        EntityKind::Text { sentence } => {
            commands.spawn(text_box::text_box_bundle(x, y, sentence, font))
        }
        EntityKind::Dynamic { .. } => {
            commands.spawn(dynamic_box::dynamic_box_bundle(
                x,
                y,
                &entity.kind,
                box_size,
            ))
        }
        EntityKind::SpeedUp { rate } => {
            commands.spawn(speedup::speedup_bundle(x, y, *rate, box_size))
        }
    }
}
