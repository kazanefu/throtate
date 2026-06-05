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

pub fn handle_hammer_input_switch(
    keys: Res<ButtonInput<KeyCode>>,
    mut hammer_action_writer: MessageWriter<HammerActionMessage>,
    mut handle_direction_writer: MessageWriter<ChangeHandleDirection>,
    config: Res<GameConfig>,
) {
    if config.input.throw.just_pressed(&keys) {
        hammer_action_writer.write(HammerActionMessage);
    }
    if config.input.ll_spin.just_pressed(&keys) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::LeftLeft));
    }
    if config.input.rr_spin.just_pressed(&keys) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::RightRight));
    }
    if config.input.lr_spin.just_pressed(&keys) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::LeftRight));
    }
    if config.input.rl_spin.just_pressed(&keys) {
        handle_direction_writer.write(ChangeHandleDirection(HandleDirection::RightLeft));
    }
}

pub fn handle_hammer_input_hold(
    keys: Res<ButtonInput<KeyCode>>,
    mut hammer_action_writer: MessageWriter<HammerActionMessage>,
    mut handle_direction_writer: MessageWriter<ChangeHandleDirection>,
    hammer_que: Query<&Hammer>,
    config: Res<GameConfig>,
) {
    for hammer in &hammer_que {
        let is_released_any_arrow_keys = config.input.ll_spin.just_released(&keys)
            || config.input.lr_spin.just_released(&keys)
            || config.input.rl_spin.just_released(&keys)
            || config.input.rr_spin.just_released(&keys);
        let is_pressed_any_arrow_keys = config.input.ll_spin.pressed(&keys)
            || config.input.lr_spin.pressed(&keys)
            || config.input.rl_spin.pressed(&keys)
            || config.input.rr_spin.pressed(&keys);

        let pressed_count = config.input.ll_spin.pressed(&keys) as u8
            + config.input.lr_spin.pressed(&keys) as u8
            + config.input.rl_spin.pressed(&keys) as u8
            + config.input.rr_spin.pressed(&keys) as u8;

        let just_pressed_count = config.input.ll_spin.just_pressed(&keys) as u8
            + config.input.lr_spin.just_pressed(&keys) as u8
            + config.input.rl_spin.just_pressed(&keys) as u8
            + config.input.rr_spin.just_pressed(&keys) as u8;

        if config.input.ll_spin.just_pressed(&keys) {
            handle_direction_writer.write(ChangeHandleDirection(HandleDirection::LeftLeft));
        }
        if config.input.rr_spin.just_pressed(&keys) {
            handle_direction_writer.write(ChangeHandleDirection(HandleDirection::RightRight));
        }
        if config.input.lr_spin.just_pressed(&keys) {
            handle_direction_writer.write(ChangeHandleDirection(HandleDirection::LeftRight));
        }
        if config.input.rl_spin.just_pressed(&keys) {
            handle_direction_writer.write(ChangeHandleDirection(HandleDirection::RightLeft));
        }
        match (
            is_pressed_any_arrow_keys,
            is_released_any_arrow_keys,
            pressed_count == just_pressed_count && just_pressed_count > 0,
            hammer.state,
        ) {
            (true, true, _, HammerState::Spinning) => {
                hammer_action_writer.write(HammerActionMessage);
                hammer_action_writer.write(HammerActionMessage);
            }
            (true, true, _, HammerState::Flying) => {
                hammer_action_writer.write(HammerActionMessage);
            }
            (false, true, _, HammerState::Spinning) => {
                hammer_action_writer.write(HammerActionMessage);
            }
            (true, false, _, HammerState::Flying) => {
                hammer_action_writer.write(HammerActionMessage);
            }
            (_, _, true, HammerState::Flying) => {
                hammer_action_writer.write(HammerActionMessage);
            }
            _ => {}
        }
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

pub fn apply_gravity_status(mut que: Query<(&mut GravityScale, &FinalStatus)>) {
    for (mut gravity_scale, status) in &mut que {
        gravity_scale.0 = status.0.gravity_scale;
    }
}

pub fn apply_restitution_status(mut que: Query<(&mut Restitution, &FinalStatus)>) {
    for (mut restitution, status) in &mut que {
        restitution.coefficient = status.0.restitution_coefficient;
    }
}
