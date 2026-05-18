use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

use crate::chaser::MainCameraChaser;

pub struct DeathEffectPlugin;

impl Plugin for DeathEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DeathVignetteMaterial>::default());

        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (update_death_effect, player_dead, update_resolution),
        );
    }
}

#[derive(ShaderType, Clone, Debug)]
struct DeathVignetteUniform {
    alpha: f32,
    resolution: Vec2,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DeathVignetteMaterial {
    #[uniform(0)]
    params: DeathVignetteUniform,
}

impl Material2d for DeathVignetteMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/death_vignette.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Component)]
struct DeathEffect;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<DeathVignetteMaterial>>,
    window: Single<&Window>,
) {
    let material = materials.add(DeathVignetteMaterial {
        params: DeathVignetteUniform {
            alpha: 0.0,
            resolution: Vec2::new(
                window.physical_width() as f32,
                window.physical_height() as f32,
            ),
        },
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10000.0, 10000.0))),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        DeathEffect,
        MainCameraChaser,
    ));
}

fn player_dead(
    mut commands: Commands,
    mut message: MessageReader<crate::action_effect::FireDeathEffect>,
) {
    for _ in message.read() {
        commands.insert_resource(DeathTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }
}

#[derive(Resource)]
struct DeathTimer(Timer);

fn update_death_effect(
    mut commands: Commands,
    time: Res<Time>,
    timer: Option<ResMut<DeathTimer>>,
    effect_query: Query<&MeshMaterial2d<DeathVignetteMaterial>, With<DeathEffect>>,
    mut materials: ResMut<Assets<DeathVignetteMaterial>>,
) {
    let Some(mut timer) = timer else {
        return;
    };

    timer.0.tick(time.delta());

    let t = timer.0.fraction();

    // 0→1→0
    let alpha = if t < 0.5 { t * 2.0 } else { (1.0 - t) * 2.0 };

    for handle in &effect_query {
        if let Some(mat) = materials.get_mut(handle) {
            mat.params.alpha = alpha;
        }
    }

    if timer.0.is_finished() {
        commands.remove_resource::<DeathTimer>();
    }
}

fn update_resolution(
    window: Single<&Window>,
    effect_query: Query<&MeshMaterial2d<DeathVignetteMaterial>, With<DeathEffect>>,
    mut materials: ResMut<Assets<DeathVignetteMaterial>>,
) {
    let resolution = Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );

    for handle in &effect_query {
        if let Some(mat) = materials.get_mut(handle) {
            mat.params.resolution = resolution;
        }
    }
}
