use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct GoalMaterialPlugin;

impl Plugin for GoalMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<GoalMaterial>::default())
            .add_systems(Update, update_goal_material);
    }
}

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct GoalUniform {
    pub time: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct GoalMaterial {
    #[uniform(0)]
    pub params: GoalUniform,
}

impl Material2d for GoalMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/goal.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

fn update_goal_material(
    time: Res<Time>,
    mut materials: ResMut<Assets<GoalMaterial>>,
    query: Query<&MeshMaterial2d<GoalMaterial>>,
) {
    for handle in &query {
        if let Some(material) = materials.get_mut(handle) {
            material.params.time = time.elapsed_secs();
        }
    }
}
