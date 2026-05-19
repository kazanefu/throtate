use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct CheckpointMaterialPlugin;

impl Plugin for CheckpointMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CheckpointMaterial>::default())
            .add_systems(Update, update_checkpoint_material);
    }
}

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct CheckpointUniform {
    pub time: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct CheckpointMaterial {
    #[uniform(0)]
    pub params: CheckpointUniform,
}

impl Material2d for CheckpointMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/checkpoint.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

fn update_checkpoint_material(
    time: Res<Time>,
    mut materials: ResMut<Assets<CheckpointMaterial>>,
    query: Query<&MeshMaterial2d<CheckpointMaterial>>,
) {
    for handle in &query {
        if let Some(material) = materials.get_mut(handle) {
            material.params.time = time.elapsed_secs();
        }
    }
}
