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
    course_materials: Res<CourseMaterials>,
    mut meshes: ResMut<Assets<Mesh>>,
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
                        &course_materials,
                        &mut meshes,
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
    course_materials: &CourseMaterials,
    meshes: &mut Assets<Mesh>,
) -> EntityCommands<'a> {
    let (x, y) = (entity.x, entity.y);
    let box_size = config.course.one_box_size;
    match &entity.kind {
        EntityKind::Ground { width, height } => {
            commands.spawn(ground::ground_bundle(x, y, *width, *height))
        }
        EntityKind::Checkpoint { priority } => commands.spawn(checkpoint::check_point_bundle(
            x,
            y,
            *priority,
            box_size,
            course_materials,
        )),
        EntityKind::Breakable { required_speed } => commands.spawn(
            breakable_box::breakable_box_bundle(x, y, *required_speed, box_size, course_materials),
        ),
        EntityKind::BreakableCustom {
            required_speed,
            width,
            height,
            rotation,
        } => commands.spawn(breakable_box::custom_breakable_bundle(
            meshes,
            x,
            y,
            *required_speed,
            *width,
            *height,
            *rotation,
            course_materials,
        )),
        EntityKind::DeathBreakable {
            required_speed,
            width,
            height,
            rotation,
        } => commands.spawn(breakable_box::death_breakable_bundle(
            meshes,
            x,
            y,
            *required_speed,
            *width,
            *height,
            box_size,
            *rotation,
            course_materials,
        )),
        EntityKind::Death => commands.spawn(death_box::death_box_bundle(
            x,
            y,
            box_size,
            course_materials,
        )),
        EntityKind::DeathCustom {
            width,
            height,
            rotation,
        } => {
            let entity_id = death_box::death_box_custom_bundle(
                commands,
                meshes,
                death_box::DeathCustomParams {
                    x,
                    y,
                    width: *width,
                    height: *height,
                    rotation: rotation.unwrap_or(0.0),
                },
                course_materials,
            );
            commands.entity(entity_id)
        }
        EntityKind::DynamicDeath { .. } => {
            let entity_id = death_box::death_box_dynamic_bundle(
                commands,
                meshes,
                x,
                y,
                &entity.kind,
                box_size,
                course_materials,
            );
            commands.entity(entity_id)
        }
        EntityKind::Turret {
            interval,
            rotation,
            bullet_lifetime,
        } => turret::spawn_turret(
            commands,
            turret::TurretSpawnParams {
                x,
                y,
                interval: *interval,
                rotation: *rotation,
                bullet_lifetime: *bullet_lifetime,
                box_size,
                course_materials,
            },
        ),
        EntityKind::Goal => commands.spawn(goal::goal_bundle(x, y, box_size, course_materials)),
        EntityKind::Text { sentence } => {
            commands.spawn(text_box::text_box_bundle(x, y, sentence, font))
        }
        EntityKind::Dynamic { .. } => commands.spawn(dynamic_box::dynamic_box_bundle(
            x,
            y,
            &entity.kind,
            box_size,
        )),
        EntityKind::SpeedUp { rate } => commands.spawn(speedup::speedup_bundle(
            x,
            y,
            *rate,
            box_size,
            course_materials,
        )),
        EntityKind::SpinVelAddWithTime { value, time } => {
            commands.spawn(spin_veladd_time::spin_vel_add_time_bundle(
                x,
                y,
                *time,
                *value,
                box_size,
                course_materials,
            ))
        }
        EntityKind::TimeLimitedBuff { buff } => {
            let entity_id = buff_with_time::spawn_time_limited_buffer(
                commands,
                x,
                y,
                *buff,
                box_size,
                course_materials,
            );
            commands.entity(entity_id)
        }
        EntityKind::WarpHole { pair_x, pair_y } => {
            // Spawn first portal at (x, y) that warps to (pair_x, pair_y)
            commands.spawn(warp_hole::warp_portal_bundle(
                x,
                y,
                *pair_x,
                *pair_y,
                box_size,
                course_materials,
            ));
            // Spawn second portal at (pair_x, pair_y) that warps back to (x, y)
            commands.spawn(warp_hole::warp_portal_bundle(
                *pair_x,
                *pair_y,
                x,
                y,
                box_size,
                course_materials,
            ))
        }
    }
}
