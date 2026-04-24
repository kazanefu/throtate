use super::*;
#[derive(Component)]
pub struct Breakable {
    required_speed: f32,
}
impl Breakable {
    pub fn new(required_speed: f32) -> Self {
        Self { required_speed }
    }
}

pub fn breakable_box_bundle(x: f32, y: f32, required_speed: f32) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        Breakable::new(required_speed),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.2),
            custom_size: Some(Vec2::new(ONE_BOX_SIZE, ONE_BOX_SIZE)),
            ..default()
        },
    )
}
