use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::state::GameState;
use crate::course::course_items::death_box::Death;

#[derive(Component)]
pub struct TurretBullet {
    pub is_active: bool,
    pub remaining_lifetime: f32,
}

pub fn spawn_inactive_bullet(
    commands: &mut Commands,
    box_size: f32,
    course_materials: &crate::course::CourseMaterials,
) -> Entity {
    commands
        .spawn((
            TurretBullet {
                is_active: false,
                remaining_lifetime: 0.0,
            },
            Death,
            Transform::from_xyz(-9999.0, -9999.0, 0.0),
            GlobalTransform::default(),
            RigidBody::Dynamic,
            Collider::cuboid(box_size / 4.0, box_size / 4.0),
            CollisionGroups::new(Group::NONE, Group::NONE),
            Mesh2d(course_materials.bullet_mesh.clone()),
            MeshMaterial2d(course_materials.bullet_material.clone()),
            Visibility::Hidden,
            Velocity::default(),
            DespawnOnExit(GameState::Playing),
        ))
        .id()
}

pub fn tick_bullets(
    time: Res<Time>,
    mut bullet_query: Query<(
        &mut TurretBullet,
        &mut Transform,
        &mut Velocity,
        &mut CollisionGroups,
        &mut Visibility,
    )>,
) {
    let delta = time.delta_secs();
    for (
        mut bullet,
        mut transform,
        mut velocity,
        mut collision_groups,
        mut visibility,
    ) in &mut bullet_query
    {
        if bullet.is_active {
            bullet.remaining_lifetime -= delta;
            if bullet.remaining_lifetime <= 0.0 {
                bullet.is_active = false;
                transform.translation = Vec3::new(-9999.0, -9999.0, 0.0);
                *velocity = Velocity::default();
                *collision_groups = CollisionGroups::new(Group::NONE, Group::NONE);
                *visibility = Visibility::Hidden;
            }
        }
    }
}
