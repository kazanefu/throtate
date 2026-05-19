use super::*;
#[derive(Component)]
pub struct CheckPoint {
    priority: u32,
}


impl CheckPoint {
    pub fn new(priority: u32) -> Self {
        Self { priority }
    }
    pub fn priority(&self) -> u32 {
        self.priority
    }
}

pub fn check_point_bundle(
    x: f32,
    y: f32,
    priority: u32,
    box_size: f32,
    course_materials: &crate::course::CourseMaterials,
) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        CheckPoint::new(priority),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Collider::cuboid(box_size / 2.0, box_size / 2.0),
        Sensor,
        Mesh2d(course_materials.checkpoint_mesh.clone()),
        MeshMaterial2d(course_materials.checkpoint_material.clone()),
    )
}
