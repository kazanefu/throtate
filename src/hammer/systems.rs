use crate::{config::GameConfig, hammer::status::FinalStatus};

use super::definition::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn update_hammer(
    mut commands: Commands,
    mut hammer_query: Query<(Entity, &mut Hammer, &FinalStatus)>,
    mut transform_query: Query<&mut Transform>,
    mut hammer_action_reader: MessageReader<HammerActionMessage>,
    config: Res<GameConfig>,
) {
    for _ in hammer_action_reader.read() {
        for (hammer_entity, mut hammer, status) in hammer_query.iter_mut() {
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
                        + (hammer_transform.1
                            * hammer.handle_direction.offset(&config.hammer).extend(0.0));
                    let (vel, stiff) = hammer.handle_direction.spin(&status.0);
                    commands.entity(hammer_entity).insert(ImpulseJoint::new(
                        hammer.pivot_entity,
                        RevoluteJointBuilder::new()
                            .local_anchor1(Vec2::ZERO)
                            .local_anchor2(hammer.handle_direction.offset(&config.hammer))
                            .motor_velocity(vel, stiff),
                    ));
                    hammer.state = HammerState::Spinning;
                }
            }
        }
    }
}

pub fn fix_hammer_z(mut q: Query<&mut Transform, With<Hammer>>) {
    for mut t in &mut q {
        t.translation.z = 10.0;
    }
}

pub fn free_hammer(
    mut commands: Commands,
    mut hammer_query: Query<(Entity, &mut Hammer)>,
    mut hammer_free_reader: MessageReader<HammerFreeMessage>,
) {
    for _ in hammer_free_reader.read() {
        for (hammer_entity, mut hammer) in &mut hammer_query {
            if matches!(hammer.state, HammerState::Spinning) {
                commands.entity(hammer_entity).remove::<ImpulseJoint>();
                hammer.state = HammerState::Flying;
            }
        }
    }
}

pub fn handle_hammer_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut hammer_action_writer: MessageWriter<HammerActionMessage>,
    mut handle_direction_writer: MessageWriter<ChangeHandleDirection>,
    config: Res<GameConfig>,
) {
    if keys.just_pressed(config.input.throw) {
        hammer_action_writer.write(HammerActionMessage);
    }
    if keys.just_pressed(config.input.ll_spin) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::LeftLeft));
    }
    if keys.just_pressed(config.input.rr_spin) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::RightRight));
    }
    if keys.just_pressed(config.input.lr_spin) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::LeftRight));
    }
    if keys.just_pressed(config.input.rl_spin) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::RightLeft));
    }
}
pub fn change_handle_direction(
    mut hammer_query: Query<&mut Hammer>,
    mut change_detection_message: MessageReader<ChangeHandleDirection>,
) {
    for message in change_detection_message.read() {
        for mut hammer in &mut hammer_query {
            hammer.handle_direction = message.0;
        }
    }
}

pub fn pivot_texture(
    mut pivot_query: Query<&mut Sprite, With<Pivot>>,
    textures: Res<PivotTextures>,
    mut handle_action_reader: MessageReader<ChangeHandleDirection>,
) {
    for ChangeHandleDirection(message) in handle_action_reader.read() {
        for mut sprite in &mut pivot_query {
            match message {
                HandleDirection::LeftLeft => {
                    sprite.image = textures.blue.clone();
                    sprite.flip_x = true;
                }
                HandleDirection::LeftRight => {
                    sprite.image = textures.blue.clone();
                    sprite.flip_x = false;
                }
                HandleDirection::RightLeft => {
                    sprite.image = textures.magenta.clone();
                    sprite.flip_x = true;
                }
                HandleDirection::RightRight => {
                    sprite.image = textures.magenta.clone();
                    sprite.flip_x = false;
                }
            }
        }
    }
}

pub fn update_hammer_state_view(
    mut visibility_que: Query<&mut Visibility, With<HammerStateView>>,
    hammer_que: Query<&Hammer, Changed<Hammer>>,
) {
    let Ok(Hammer {
        state: hammer_state,
        ..
    }) = hammer_que.single()
    else {
        return;
    };
    for mut visibility in &mut visibility_que {
        *visibility = match hammer_state {
            HammerState::Flying => Visibility::Hidden,
            HammerState::Spinning => Visibility::Visible,
        };
    }
}
