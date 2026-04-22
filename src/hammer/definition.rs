use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const HAMMER_HANDLE_OFFSET: Vec2 = Vec2 { x: -40.0, y: 0.0 };
pub const HAMMER_ACTION_KEY_CODE: KeyCode = KeyCode::Space;
pub const CHANGE_DIRECTION_KEY_CODE_LL: KeyCode = KeyCode::ArrowLeft;
pub const CHANGE_DIRECTION_KEY_CODE_RR: KeyCode = KeyCode::ArrowRight;
pub const CHANGE_DIRECTION_KEY_CODE_LR: KeyCode = KeyCode::ArrowDown;
pub const CHANGE_DIRECTION_KEY_CODE_RL: KeyCode = KeyCode::ArrowUp;
pub const HAMMER_SPIN: (f32, f32) = (100.0, 0.1);

#[derive(Clone, Copy)]
pub enum HammerState {
    Spinning,
    Flying,
}

#[derive(Message)]
pub struct ChangeHandleDirection(pub HandleDirection);

#[derive(Clone, Copy)]
pub enum HandleDirection {
    LeftLeft,
    RightRight,
    LeftRight,
    RightLeft,
}
impl HandleDirection {
    pub fn offset(&self) -> Vec2 {
        match self {
            Self::LeftLeft => HAMMER_HANDLE_OFFSET,
            Self::RightRight => HAMMER_HANDLE_OFFSET * -1.0,
            Self::LeftRight => HAMMER_HANDLE_OFFSET,
            Self::RightLeft => HAMMER_HANDLE_OFFSET * -1.0,
        }
    }
    pub fn spin(&self) -> (f32, f32) {
        match self {
            Self::LeftLeft => HAMMER_SPIN,
            Self::RightRight => (-HAMMER_SPIN.0, HAMMER_SPIN.1),
            Self::LeftRight => (-HAMMER_SPIN.0, HAMMER_SPIN.1),
            Self::RightLeft => HAMMER_SPIN,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Hammer {
    pub pivot_entity: Entity,
    pub state: HammerState,
    pub handle_direction: HandleDirection,
}

#[derive(Component)]
pub struct Pivot;

#[derive(Message)]
pub struct HammerActionMessage;

pub fn hammer_bundle(pivot_entity: Entity, translate: Vec2) -> impl Bundle {
    (
        Hammer {
            pivot_entity,
            state: HammerState::Spinning,
            handle_direction: HandleDirection::LeftLeft,
        },
        RigidBody::Dynamic,
        Transform::from_xyz(translate.x, translate.y, 0.0),
        Collider::ball(10.0),
        ImpulseJoint::new(
            pivot_entity,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::ZERO)
                .local_anchor2(HAMMER_HANDLE_OFFSET)
                .motor_velocity(HAMMER_SPIN.0, HAMMER_SPIN.1),
        ),
        Sprite {
            color: Color::srgb(0.0, 0.4, 0.9),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
    )
}