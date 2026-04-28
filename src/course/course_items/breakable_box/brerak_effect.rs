use bevy::{asset::{Assets, Handle}, ecs::{message::{Message, MessageReader}, resource::Resource, system::Commands}, math::{Vec3, Vec4}, prelude::{Res,ResMut}, transform::components::Transform};

use bevy_hanabi::{
    Attribute, ColorBlendMask, ColorBlendMode, ColorOverLifetimeModifier, EffectAsset, Module,
    ParticleEffect, SetAttributeModifier, SetPositionCircleModifier, SetVelocityCircleModifier,
    ShapeDimension, SizeOverLifetimeModifier, SpawnerSettings,
};

use crate::DespawnWithTime;

#[derive(Message)]
pub struct FireBreakEffect(pub Vec3);

pub fn handle_break_effect(
    mut commands: Commands,
    effect: Res<BreakEffect>,
    mut fire_message: MessageReader<FireBreakEffect>,
) {
    for position in fire_message.read() {
        commands.spawn((
            ParticleEffect::new(effect.0.clone().expect("break effect never setuped")),
            Transform::from_translation(position.0),
            DespawnWithTime(2.0),
        ));
    }
}

#[derive(Resource, Clone)]
pub struct BreakEffect(pub Option<Handle<EffectAsset>>);

pub fn setup_break_effect(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut break_effect_res: ResMut<BreakEffect>,
) {
    let handle = break_effect(&mut effects);
    break_effect_res.0 = Some(handle);
}

// break effect
pub fn break_effect(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 1.0, 0.0, 1.));
    gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.01));

    let mut module = Module::default();

    let init_pos = SetPositionCircleModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(30.0),
        axis: module.lit(Vec3::Z),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Z),
        speed: module.lit(100.),
    };

    let lifetime = module.lit(1.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        SpawnerSettings::once(100.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("berak_effect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .render(ColorOverLifetimeModifier {
        gradient, // rust-analyzer says `expected Gradient, found Gradient<Vec4>` but that is just a bug of rust-analyzer.  this code works!
        blend: ColorBlendMode::Modulate,
        mask: ColorBlendMask::RGBA,
    })
    .render(SizeOverLifetimeModifier {
        // rust-analyzer says `expected Gradient, found Gradient<Vec3>` but that is just a bug of rust-analyzer. this code works!
        gradient: bevy_hanabi::Gradient::from_keys(vec![
            (0.0, Vec3::splat(15.0)),
            (1.0, Vec3::splat(0.0)),
        ]),
        screen_space_size: false,
    });

    effects.add(effect)
}
