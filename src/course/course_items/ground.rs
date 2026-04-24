use bevy::prelude::*;
use bevy_rapier2d::prelude::*;



#[derive(Component)]
pub struct Ground;

pub fn ground_bundle(x: f32, y: f32, width: f32, height: f32) -> impl Bundle {
    (
        Transform::from_xyz(x, y, 0.0),
        Ground,
        RigidBody::Fixed,
        Collider::cuboid(width / 2.0, height / 2.0),
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.2),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
    )
}