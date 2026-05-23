use super::*;
use crate::{
    LifeTime,
    collision::get_contained_entity,
    hammer::{Buff, BuffStatusChannel, BuffType, FinalStatus},
    state::GameState,
};
use std::collections::HashMap;

pub struct SpinVelAddTimePlugin;

impl Plugin for SpinVelAddTimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpinVelBuffMap>()
            .add_message::<SpawnSpinVelAddBuff>()
            .add_systems(OnExit(GameState::Playing), clear_buff_map)
            .add_systems(Update, (collision_system, spawn_spin_vel_add_buff).chain());
    }
}

#[derive(Resource, Default)]
struct SpinVelBuffMap(HashMap<Entity, Entity>);

fn clear_buff_map(mut map: ResMut<SpinVelBuffMap>) {
    map.0.clear();
}

#[derive(Component)]
struct SpinVelAddTime {
    life_time: f32,
    spin_vel: f32,
}

impl SpinVelAddTime {
    pub fn new(life_time: f32, spin_vel: f32) -> Self {
        Self {
            life_time,
            spin_vel,
        }
    }
}

pub fn spin_vel_add_time_bundle(
    x: f32,
    y: f32,
    life_time: f32,
    spin_vel: f32,
    box_size: f32,
    course_materials: &crate::course::CourseMaterials,
) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        SpinVelAddTime::new(life_time, spin_vel),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(box_size / 2.0, box_size / 2.0),
        Sensor,
        Mesh2d(course_materials.speedup_mesh.clone()),
        MeshMaterial2d(course_materials.speedup_material.clone()),
    )
}

#[derive(Message)]
struct SpawnSpinVelAddBuff {
    life_time: f32,
    spin_vel: f32,
    target: Entity,
    id: Entity,
}

fn spawn_spin_vel_add_buff(
    mut commands: Commands,
    mut msg: MessageReader<SpawnSpinVelAddBuff>,
    mut map: ResMut<SpinVelBuffMap>,
) {
    for SpawnSpinVelAddBuff {
        life_time,
        spin_vel,
        target,
        id,
    } in msg.read()
    {
        // 既存Buff削除
        if let Some(old_buff) = map.0.remove(id) {
            commands.entity(old_buff).despawn();
        }

        // 新Buff生成
        let buff_entity = commands
            .spawn((
                LifeTime::new(*life_time),
                Buff {
                    channel: BuffStatusChannel::SpinVelocity,
                    ty: BuffType::Add,
                    target: *target,
                    value: Some(*spin_vel),
                    priority: 0,
                },
            ))
            .id();

        map.0.insert(*id, buff_entity);
    }
}

fn collision_system(
    mut msg: MessageWriter<SpawnSpinVelAddBuff>,
    mut collision_events: MessageReader<CollisionEvent>,
    status_holder_que: Query<Entity, With<FinalStatus>>,
    buffer_que: Query<&SpinVelAddTime>,
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
            msg.write(SpawnSpinVelAddBuff {
                life_time: buffer.life_time,
                spin_vel: buffer.spin_vel,
                target: player,
                id: buffer_entity,
            });
        }
    }
}
