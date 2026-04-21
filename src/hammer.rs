use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const HAMMER_HANDLE_OFFSET: Vec2 = Vec2 { x: -40.0, y: 0.0 };
const HAMMER_ACTION_KEY_CODE: KeyCode = KeyCode::Space;
const HAMMER_SPIN:(f32,f32) = (10.0,1.0);
pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<HammerActionMessage>()
            .add_systems(Update, (handle_hammer_input, update_hammer));
    }
}

#[derive(Clone, Copy)]
enum HammerState {
    Spinning,
    Flying,
}

#[derive(Component, Clone, Copy)]
pub struct Hammer {
    pivot_entity: Entity,
    state: HammerState,
}

#[derive(Component)]
pub struct Pivot;

#[derive(Message)]
struct HammerActionMessage;

pub fn hammer_bundle(pivot_entity: Entity, translate: Vec2) -> impl Bundle {
    (
        Hammer {
            pivot_entity,
            state: HammerState::Spinning,
        },
        RigidBody::Dynamic,
        Transform::from_xyz(translate.x, translate.y, 0.0),
        Collider::ball(10.0),
        ImpulseJoint::new(
            pivot_entity,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::ZERO)
                .local_anchor2(HAMMER_HANDLE_OFFSET)
                .motor_velocity(HAMMER_SPIN.0,HAMMER_SPIN.1),
        ),
        Sprite {
            color: Color::srgb(0.0, 0.4, 0.9),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
    )
}

#[allow(unused)]
pub fn spawn_hammer<'a>(commands: &'a mut Commands, translate: Vec2) -> EntityCommands<'a> {
    let pivot = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(translate.x - 1.0, translate.y - 1.0, 0.0),
            Pivot,
        ))
        .id();
    commands.spawn(hammer_bundle(pivot, translate))
}

fn update_hammer(
    mut commands: Commands,
    mut hammer_query: Query<(Entity, &mut Hammer)>,
    mut transform_query: Query<&mut Transform>,
    mut hammer_action_reader: MessageReader<HammerActionMessage>,
) {
    for _ in hammer_action_reader.read() {
        for (hammer_entity, mut hammer) in hammer_query.iter_mut() {
            let hammer_transform = {
                let hammer_transform = transform_query
                    .get(hammer_entity)
                    .expect("hammer has no transform");
                (hammer_transform.translation, hammer_transform.rotation)
            };
            match hammer.state {
                HammerState::Spinning => {
                    commands.entity(hammer_entity).remove::<ImpulseJoint>();
                    hammer.state = HammerState::Flying;
                }
                HammerState::Flying => {
                    let mut pivot_transform = transform_query
                        .get_mut(hammer.pivot_entity)
                        .expect("This hammer has no pivot");
                    pivot_transform.translation = hammer_transform.0
                        + (hammer_transform.1 * HAMMER_HANDLE_OFFSET.extend(0.0));
                    commands.entity(hammer_entity).insert(ImpulseJoint::new(
                        hammer.pivot_entity,
                        RevoluteJointBuilder::new()
                            .local_anchor1(Vec2::ZERO)
                            .local_anchor2(HAMMER_HANDLE_OFFSET)
                            .motor_velocity(HAMMER_SPIN.0,HAMMER_SPIN.1),
                    ));
                    hammer.state = HammerState::Spinning;
                }
            }
        }
    }
}

fn handle_hammer_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut hammer_action_writer: MessageWriter<HammerActionMessage>,
) {
    if keys.just_pressed(HAMMER_ACTION_KEY_CODE) {
        hammer_action_writer.write(HammerActionMessage);
    }
}
