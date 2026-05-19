use super::*;
#[derive(Component)]
pub struct Death;

pub fn death_box_bundle(
    x: f32,
    y: f32,
    box_size: f32,
    course_materials: &crate::course::CourseMaterials,
) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        Death,
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(box_size / 2.0, box_size / 2.0),
        Mesh2d(course_materials.death_mesh.clone()),
        MeshMaterial2d(course_materials.death_material.clone()),
    )
}
