use super::*;
use crate::{
    LifeTime,
    collision::get_contained_entity,
    hammer::{Buff, BuffStatusChannel, BuffType, FinalStatus},
    state::GameState,
};
use serde::Deserialize;
use std::collections::HashMap;

pub struct TimeLimitedBuffPlugin;

impl Plugin for TimeLimitedBuffPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeLimitedBuffMap>()
            .add_message::<SpawnTimeLimitedBuff>()
            .add_systems(OnExit(GameState::Playing), clear_buff_map)
            .add_systems(Update, (collision_system, spawn_time_limited_buff));
    }
}

#[derive(Resource, Default)]
struct TimeLimitedBuffMap(HashMap<Entity, Entity>);

fn clear_buff_map(mut map: ResMut<TimeLimitedBuffMap>) {
    map.0.clear();
}

#[derive(Component, Deserialize, Clone, Copy)]
pub struct TimeLimitedBuffer {
    life_time: f32,
    channel: BuffStatusChannel,
    buff_type: BuffType,
    value: Option<f32>,
}

pub fn time_limited_buffer_bundle(
    x: f32,
    y: f32,
    buffer: TimeLimitedBuffer,
    box_size: f32,
    course_materials: &crate::course::CourseMaterials,
) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        buffer,
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(box_size / 2.0, box_size / 2.0),
        Sensor,
        Mesh2d(),
        MeshMaterial2d(),
    )
}

#[derive(Message)]
struct SpawnTimeLimitedBuff {
    buff: TimeLimitedBuffer,
    target: Entity,
    id: Entity,
}

fn spawn_time_limited_buff(
    mut commands: Commands,
    mut msg: MessageReader<SpawnTimeLimitedBuff>,
    mut map: ResMut<TimeLimitedBuffMap>,
) {
    for SpawnTimeLimitedBuff { buff, target, id } in msg.read() {
        if let Some(old_buff) = map.0.remove(id) {
            commands.entity(old_buff).despawn();
        }

        let buff_entity = commands
            .spawn((
                LifeTime::new(*&buff.life_time),
                Buff {
                    channel: buff.channel,
                    ty: buff.buff_type,
                    target: *target,
                    value: buff.value,
                    priority: 0,
                },
            ))
            .id();
        map.0.insert(*id, buff_entity);
    }
}

fn collision_system(
    mut msg: MessageWriter<SpawnTimeLimitedBuff>,
    mut collision_events: MessageReader<CollisionEvent>,
    status_holder_que: Query<Entity, With<FinalStatus>>,
    buffer_que: Query<&TimeLimitedBuffer>,
) {
    for &event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let Some(player) = get_contained_entity(e1, e2, &status_holder_que) else {
                continue;
            };
            let Some(buffer_entity) = get_contained_entity(e1, e2, &buffer_que) else {
                continue;
            };
            let Ok(buffer) = buffer_que.get(buffer_entity) else {
                continue;
            };
            msg.write(SpawnTimeLimitedBuff {
                buff: *buffer,
                target: player,
                id: buffer_entity,
            });
        }
    }
}
