use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct TurretMaterialPlugin;

impl Plugin for TurretMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<TurretMaterial>::default())
            .add_systems(Update, update_turret_material);
    }
}

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct TurretUniform {
    pub time: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct TurretMaterial {
    #[uniform(0)]
    pub params: TurretUniform,
}

impl Material2d for TurretMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/turret.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

fn update_turret_material(
    time: Res<Time>,
    mut materials: ResMut<Assets<TurretMaterial>>,
    query: Query<&MeshMaterial2d<TurretMaterial>>,
) {
    for handle in &query {
        if let Some(material) = materials.get_mut(handle) {
            material.params.time = time.elapsed_secs();
        }
    }
}
