use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

use crate::state::GameState;

pub struct TransitionMaterialPlugin;

impl Plugin for TransitionMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<TransitionMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Playing), start_playing_transition)
            .add_systems(OnEnter(GameState::Result), start_result_transition)
            .add_systems(
                OnEnter(GameState::CourseSelection),
                start_course_selection_transition,
            )
            .add_systems(Update, update_transition);
    }
}

#[derive(ShaderType, Clone, Debug)]
struct TransitionUniform {
    base_color: Vec4,
    quad_center: Vec2,
    progress: f32,
    _padding: f32,
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
                quad_center: Vec2::ZERO,
                progress: 999.0,
                _padding: 0.0,
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

#[derive(Clone, Copy)]
enum TransitionDirection {
    Shrink,
    Expand,
}

#[derive(Resource)]
struct TransitionState {
    timer: Timer,
    color: Color,
    direction: TransitionDirection,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TransitionMaterial>>,
) {
    let material = materials.add(TransitionMaterial::from_color(Color::WHITE));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10000.0, 10000.0))),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 100.0),
        GameTransitionEffect,
    ));
}

fn start_transition(commands: &mut Commands, color: Color, direction: TransitionDirection) {
    commands.insert_resource(TransitionState {
        timer: Timer::from_seconds(1.2, TimerMode::Once),
        color,
        direction,
    });
}

fn start_playing_transition(mut commands: Commands) {
    start_transition(
        &mut commands,
        Color::srgb(0.0, 0.6, 1.0),
        TransitionDirection::Shrink,
    );
}

fn start_result_transition(mut commands: Commands) {
    start_transition(
        &mut commands,
        Color::srgb(1.0, 0.0, 1.0),
        TransitionDirection::Shrink,
    );
}

fn start_course_selection_transition(mut commands: Commands) {
    start_transition(
        &mut commands,
        Color::srgb(1.0, 1.0, 0.0),
        TransitionDirection::Expand,
    );
}

fn update_transition(
    mut commands: Commands,
    time: Res<Time>,
    transition: Option<ResMut<TransitionState>>,
    effect_query: Query<
        (&MeshMaterial2d<TransitionMaterial>, &GlobalTransform),
        With<GameTransitionEffect>,
    >,
    mut materials: ResMut<Assets<TransitionMaterial>>,
) {
    let Some(mut transition) = transition else {
        return;
    };

    transition.timer.tick(time.delta());

    let fraction = transition.timer.fraction();

    let progress = match transition.direction {
        TransitionDirection::Shrink => fraction,
        TransitionDirection::Expand => 1.0 - fraction,
    };

    for (handle, transform) in &effect_query {
        if let Some(mat) = materials.get_mut(handle) {
            mat.params.progress = progress;
            mat.params.base_color = transition.color.to_linear().to_vec4();
            mat.params.quad_center = transform.translation().truncate();
        }
    }

    if transition.timer.is_finished() {
        for (handle, _) in &effect_query {
            if let Some(mat) = materials.get_mut(handle) {
                mat.params.progress = 999.0;
            }
        }

        commands.remove_resource::<TransitionState>();
    }
}
