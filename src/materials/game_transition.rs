use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

use crate::chaser::MainCameraChaser;
use crate::state::GameState;

pub struct TransitionMaterialPlugin;

impl Plugin for TransitionMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<TransitionMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Playing), start_transition)
            .add_systems(Update, update_transition);
    }
}

#[derive(ShaderType, Clone, Debug)]
struct TransitionUniform {
    base_color: Vec4,
    progress: f32,
    _padding: Vec3,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TransitionMaterial {
    #[uniform(0)]
    params: TransitionUniform,
}

impl TransitionMaterial {
    pub fn from_color(color: Color) -> Self {
        Self {
            params: TransitionUniform {
                base_color: color.to_linear().to_vec4(),
                progress: 999.0,
                _padding: Vec3::ZERO,
            },
        }
    }
}

impl Material2d for TransitionMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/game_transition.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Component)]
struct GameTransitionEffect;

#[derive(Resource)]
struct TransitionTimer(Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TransitionMaterial>>,
) {
    let material = materials.add(TransitionMaterial::from_color(Color::srgb(0.0, 0.6, 1.0)));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10000.0, 10000.0))),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 100.0),
        MainCameraChaser,
        GameTransitionEffect,
    ));
}

fn start_transition(mut commands: Commands) {
    commands.insert_resource(TransitionTimer(Timer::from_seconds(1.2, TimerMode::Once)));
}

fn update_transition(
    mut commands: Commands,
    time: Res<Time>,
    timer: Option<ResMut<TransitionTimer>>,
    effect_query: Query<&MeshMaterial2d<TransitionMaterial>, With<GameTransitionEffect>>,
    mut materials: ResMut<Assets<TransitionMaterial>>,
) {
    let Some(mut timer) = timer else {
        return;
    };

    timer.0.tick(time.delta());

    let progress = timer.0.fraction();

    for handle in &effect_query {
        if let Some(mat) = materials.get_mut(handle) {
            mat.params.progress = progress;
        }
    }

    if timer.0.is_finished() {
        for handle in &effect_query {
            if let Some(mat) = materials.get_mut(handle) {
                mat.params.progress = 999.0;
            }
        }

        commands.remove_resource::<TransitionTimer>();
    }
}
