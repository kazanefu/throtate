use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::DespawnWithTime;

pub struct ActionEffectPlugin;

impl Plugin for ActionEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FireDeathEffect>()
        .add_message::<FireCheckPointEffect>()
        .insert_resource(DeathEffect(None))
        .insert_resource(CheckPointEffect(None))
        .add_systems(Startup,(setup_death_effect,setup_checkpoint_effect))
        .add_systems(Update,(handle_death_effect,handle_checkpoint_effect));
    }
}


// death effect

#[derive(Message)]
pub struct FireDeathEffect(pub Vec3);

#[derive(Resource, Clone)]
pub struct DeathEffect(pub Option<Handle<EffectAsset>>);

pub fn handle_death_effect(
    mut commands: Commands,
    effect: Res<DeathEffect>,
    mut fire_message: MessageReader<FireDeathEffect>,
) {
    for position in fire_message.read() {
        commands.spawn((
            ParticleEffect::new(effect.0.clone().expect("death effect never setuped")),
            Transform::from_translation(position.0),
            DespawnWithTime(1.0),
        ));
    }
}

pub fn setup_death_effect(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut death_effect_res: ResMut<DeathEffect>,
) {
    let handle = death_effect(&mut effects);
    death_effect_res.0 = Some(handle);
}
fn death_effect(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.0, 0.0, 1.));
    gradient.add_key(1.0, Vec4::new(1.0, 1.0, 0.0, 0.01));

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
        speed: module.lit(1000.),
    };

    let lifetime = module.lit(1.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        SpawnerSettings::once(10.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("death_effect")
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
            (0.0, Vec3::splat(20.0)),
            (1.0, Vec3::splat(0.0)),
        ]),
        screen_space_size: false,
    });

    effects.add(effect)
}



// Checkpoint effect

#[derive(Message)]
pub struct FireCheckPointEffect(pub Vec3);

#[derive(Resource, Clone)]
pub struct CheckPointEffect(pub Option<Handle<EffectAsset>>);

pub fn handle_checkpoint_effect(
    mut commands: Commands,
    effect: Res<CheckPointEffect>,
    mut fire_message: MessageReader<FireCheckPointEffect>
) {
    for position in fire_message.read() {
        commands.spawn((
            ParticleEffect::new(effect.0.clone().expect("checkpoint effect never setuped")),
            Transform::from_translation(position.0),
            DespawnWithTime(5.0),
        ));
    }
}

pub fn setup_checkpoint_effect(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut checkpoint_effect_res: ResMut<CheckPointEffect>,
) {
    let handle = checkpoint_effect(&mut effects);
    checkpoint_effect_res.0 = Some(handle);
}

pub fn checkpoint_effect(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut gradient: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.0, 0.4, 0.8, 1.));
    gradient.add_key(1.0, Vec4::new(0.0, 1.0, 1.0, 0.3));

    let mut module = Module::default();

    let init_pos = SetPositionCircleModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(30.0),
        axis: module.lit(Vec3::Z),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocityCircleModifier {
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Z),
        speed: module.lit(30.0),
    };

    let lifetime = module.lit(3.);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let effect = EffectAsset::new(
        32768,
        SpawnerSettings::rate(10.0.into()).with_spawn_duration(1.0.into()).with_cycle_count(1),
        module,
    )
    .with_name("checkpoint_effect")
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