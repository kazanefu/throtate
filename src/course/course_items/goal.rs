use super::*;
#[derive(Component)]
pub struct Goal;

pub fn goal_bundle(
    x: f32,
    y: f32,
    box_size: f32,
    course_materials: &crate::course::CourseMaterials,
) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        Goal,
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(box_size / 2.0, box_size / 2.0),
        Mesh2d(course_materials.goal_mesh.clone()),
        MeshMaterial2d(course_materials.goal_material.clone()),
    )
}
