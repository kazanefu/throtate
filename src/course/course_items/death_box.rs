use super::*;
use crate::course::EntityKind;

#[derive(Component)]
pub struct Death;

pub struct DeathCustomParams {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
}

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

pub fn death_box_custom_bundle(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    params: DeathCustomParams,
    course_materials: &crate::course::CourseMaterials,
) -> Entity {
    let mesh = meshes.add(Rectangle::new(params.width, params.height));
    commands
        .spawn((
            Transform::from_xyz(params.x, params.y, 0.0)
                .with_rotation(Quat::from_rotation_z(params.rotation)),
            Death,
            RigidBody::Fixed,
            ActiveEvents::COLLISION_EVENTS,
            Collider::cuboid(params.width / 2.0, params.height / 2.0),
            Mesh2d(mesh),
            MeshMaterial2d(course_materials.death_material.clone()),
        ))
        .id()
}

pub fn death_box_dynamic_bundle(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    x: f32,
    y: f32,
    kind: &EntityKind,
    box_size: f32,
    course_materials: &crate::course::CourseMaterials,
) -> Entity {
    let (
        width,
        height,
        rotation,
        gravity_scale,
        linear_damping,
        angular_damping,
        density_scale,
        restitution_coefficient,
    ) = match kind {
        EntityKind::DynamicDeath {
            width,
            height,
            rotation,
            gravity_scale,
            linear_damping,
            angular_damping,
            density_scale,
            restitution_coefficient,
        } => (
            width.unwrap_or(box_size),
            height.unwrap_or(box_size),
            rotation.unwrap_or(0.0),
            gravity_scale.unwrap_or(1.0),
            linear_damping.unwrap_or(0.0),
            angular_damping.unwrap_or(0.0),
            density_scale.unwrap_or(1.0),
            restitution_coefficient.unwrap_or(0.0),
        ),
        _ => {
            panic!("kind must be DynamicDeath");
        }
    };

    let mesh = meshes.add(Rectangle::new(width, height));
    commands
        .spawn((
            Transform::from_xyz(x, y, 0.0).with_rotation(Quat::from_rotation_z(rotation)),
            Death,
            RigidBody::Dynamic,
            ActiveEvents::COLLISION_EVENTS,
            Collider::cuboid(width / 2.0, height / 2.0),
            Mesh2d(mesh),
            MeshMaterial2d(course_materials.death_material.clone()),
            GravityScale(gravity_scale),
            Damping {
                linear_damping,
                angular_damping,
            },
            Velocity::default(),
            TransformInterpolation::default(),
            Restitution::coefficient(restitution_coefficient),
            ColliderMassProperties::Density(density_scale),
        ))
        .id()
}
