use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct DeathMaterialPlugin;

impl Plugin for DeathMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DeathMaterial>::default())
            .add_systems(Update, update_death_material);
    }
}

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct DeathUniform {
    pub time: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct DeathMaterial {
    #[uniform(0)]
    pub params: DeathUniform,
}

impl Material2d for DeathMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/death.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

fn update_death_material(
    time: Res<Time>,
    mut materials: ResMut<Assets<DeathMaterial>>,
    query: Query<&MeshMaterial2d<DeathMaterial>>,
) {
    for handle in &query {
        if let Some(material) = materials.get_mut(handle) {
            material.params.time = time.elapsed_secs();
        }
    }
}
