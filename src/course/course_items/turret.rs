use super::*;
#[derive(Component)]
pub struct Turret;

pub fn turret_bundle(x: f32, y: f32, interval: f32, rotation: f32) -> impl Bundle {
    (
        Transform {
            translation: Vec3::new(x, y, 0.0),
            rotation: Quat::from_rotation_z(rotation),
            scale: Vec3::ONE,
        },
        crate::utils::Interval {
            time: 0.0,
            interval,
        },
        Turret,
        RigidBody::Fixed,
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(ONE_BOX_SIZE, ONE_BOX_SIZE)),
            ..default()
        },
    )
}
