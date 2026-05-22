use bevy::prelude::*;

use crate::config::GameConfig;

#[derive(Clone, Copy, Default)]
pub struct HammerStatus {
    pub gravity_scale: f32,
    pub restitution_coefficient: f32,
    pub spin_velocity: f32,
    pub spin_stiffness: f32,
}

#[derive(Clone, Copy)]
struct StatusSum {
    add: f32,
    muladd: f32,
    mul: f32,
    abs: Option<f32>,
}

impl Default for StatusSum {
    fn default() -> Self {
        Self {
            add: 0.0,
            muladd: 0.0,
            mul: 1.0,
            abs: None,
        }
    }
}

#[derive(Default, Clone, Copy)]
struct HammesStatusSum {
    pub gravity_scale: StatusSum,
    pub restitution_coefficient: StatusSum,
    pub spin_velocity: StatusSum,
    pub spin_stiffness: StatusSum,
}

#[derive(Component, Clone, Copy, Default)]
pub struct FinalStatus(pub HammerStatus);

#[derive(Component, Clone, Copy, Default)]
pub struct BaseStatus(pub HammerStatus);

#[derive(Bundle, Default)]
pub struct StatusHolder {
    base_status: BaseStatus,
    final_status: FinalStatus,
}

pub fn init_base_status(
    config: Res<GameConfig>,
    mut base_status_que: Query<&mut BaseStatus, Added<BaseStatus>>,
) {
    for mut base_status in &mut base_status_que {
        base_status.0 = HammerStatus {
            gravity_scale: 1.0,
            restitution_coefficient: config.hammer.restitution_coefficient,
            spin_velocity: config.hammer.spin_velocity,
            spin_stiffness: config.hammer.spin_stiffness,
        }
    }
}

#[allow(unused)]
#[derive(Component, Clone, Copy, Debug)]
pub enum BuffStatusChannel {
    SpinVelocity,
    SpinStiffness,
    GravityScale,
    RestitutionCefficient,
}

// Add -> Mul -> Abs
// (base * Add) * Mul or Abs
#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub enum BuffType {
    Add,
    MulAdd,
    Mul,
    Abs,
}

#[derive(Component, Clone)]
pub struct Buff {
    pub channel: BuffStatusChannel,
    pub ty: BuffType,
    pub target: Entity,
    pub value: Option<f32>,
}

fn cal_each_status(sum: StatusSum, base: f32) -> f32 {
    match sum.abs {
        Some(a) => a,
        None => (base + sum.add) * (sum.muladd + sum.mul),
    }
}

fn cal_final_status(sum: HammesStatusSum, base: HammerStatus) -> HammerStatus {
    HammerStatus {
        gravity_scale: cal_each_status(sum.gravity_scale, base.gravity_scale),
        restitution_coefficient: cal_each_status(
            sum.restitution_coefficient,
            base.restitution_coefficient,
        ),
        spin_velocity: cal_each_status(sum.spin_velocity, base.spin_velocity),
        spin_stiffness: cal_each_status(sum.spin_stiffness, base.spin_stiffness),
    }
}

pub fn apply_buff(
    mut status_que: Query<(Entity, &mut FinalStatus, &BaseStatus)>,
    buff_que: Query<&Buff>,
) {
    for (entity, mut final_status, base_status) in &mut status_que {
        let mut sum = HammesStatusSum::default();
        for buff in buff_que.iter().filter(|b| b.target == entity) {
            if buff.value.is_none() {
                continue;
            }
            let channel_sum = match &buff.channel {
                BuffStatusChannel::GravityScale => &mut sum.gravity_scale,
                BuffStatusChannel::RestitutionCefficient => &mut sum.restitution_coefficient,
                BuffStatusChannel::SpinStiffness => &mut sum.spin_stiffness,
                BuffStatusChannel::SpinVelocity => &mut sum.spin_velocity,
            };
            match buff.ty {
                BuffType::Add => channel_sum.add += buff.value.unwrap(),
                BuffType::MulAdd => channel_sum.muladd += buff.value.unwrap(),
                BuffType::Mul => channel_sum.mul *= buff.value.unwrap(),
                BuffType::Abs => channel_sum.abs = buff.value,
            }
        }
        final_status.0 = cal_final_status(sum, base_status.0);
    }
}
