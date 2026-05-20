use crate::course::EntityKind;

use super::*;

pub fn dynamic_box_bundle(x: f32, y: f32, kind: &EntityKind, box_size: f32) -> impl Bundle {
    let (
        width,
        height,
        gravity_scale,
        linear_damping,
        angular_damping,
        density_scale,
        restitution_coefficient,
    ) = match kind {
        EntityKind::Dynamic {
            width,
            height,
            gravity_scale,
            linear_damping,
            angular_damping,
            density_scale,
            restitution_coefficient,
        } => (
            width.unwrap_or(box_size),
            height.unwrap_or(box_size),
            gravity_scale.unwrap_or(1.0),
            linear_damping.unwrap_or(0.0),
            angular_damping.unwrap_or(0.0),
            density_scale.unwrap_or(1.0),
            restitution_coefficient.unwrap_or(0.0),
        ),
        _ => {
            panic!("kind must be dynamic");
        }
    };
    (
        Transform::from_xyz(x, y, 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(width / 2.0, height / 2.0),
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.4),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        GravityScale(gravity_scale),
        Damping {
            linear_damping,
            angular_damping,
        },
        Velocity::default(),
        TransformInterpolation::default(),
        Restitution::coefficient(restitution_coefficient),
        ColliderMassProperties::Density(density_scale),
    )
}
