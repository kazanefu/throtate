use super::*;
#[derive(Component)]
pub struct CheckPoint {
    priority: u32,
}
impl CheckPoint {
    pub fn new(priority: u32) -> Self {
        Self { priority }
    }
}

pub fn check_point_bundle(x: f32, y: f32, priority: u32) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        CheckPoint::new(priority),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Sensor,
        Sprite {
            color: Color::srgb(0.2, 0.9, 0.9),
            custom_size: Some(Vec2::new(ONE_BOX_SIZE, ONE_BOX_SIZE)),
            ..default()
        },
    )
}
