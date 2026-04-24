use super::*;
#[derive(Component)]
pub struct Death;

pub fn death_box_bundle(x: f32, y: f32) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        Death,
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Sprite {
            color: Color::srgb(0.9, 0.2, 0.2),
            custom_size: Some(Vec2::new(ONE_BOX_SIZE, ONE_BOX_SIZE)),
            ..default()
        },
    )
}
