use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct WarpMaterialPlugin;

impl Plugin for WarpMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<WarpMaterial>::default())
            .add_systems(Update, update_warp_material);
    }
}

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct WarpUniform {
    pub time: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct WarpMaterial {
    #[uniform(0)]
    pub params: WarpUniform,
}

impl Material2d for WarpMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/warp.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

fn update_warp_material(
    time: Res<Time>,
    mut materials: ResMut<Assets<WarpMaterial>>,
    query: Query<&MeshMaterial2d<WarpMaterial>>,
) {
    for handle in &query {
        if let Some(material) = materials.get_mut(handle) {
            material.params.time = time.elapsed_secs();
        }
    }
}
