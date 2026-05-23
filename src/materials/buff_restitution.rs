use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct BuffRestitutionMaterialPlugin;

impl Plugin for BuffRestitutionMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BuffRestitutionMaterial>::default())
            .add_systems(Update, update_buff_restitution_material);
    }
}

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct BuffRestitutionUniform {
    pub time: f32,
    pub color: Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct BuffRestitutionMaterial {
    #[uniform(0)]
    pub params: BuffRestitutionUniform,
}

impl BuffRestitutionMaterial {
    pub fn new(color: Color) -> Self {
        Self {
            params: BuffRestitutionUniform {
                time: 0.0,
                color: color.to_linear().to_vec4(),
            },
        }
    }
}

impl Material2d for BuffRestitutionMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/buff_restitution.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

fn update_buff_restitution_material(
    time: Res<Time>,
    mut materials: ResMut<Assets<BuffRestitutionMaterial>>,
    query: Query<&MeshMaterial2d<BuffRestitutionMaterial>>,
) {
    for handle in &query {
        if let Some(material) = materials.get_mut(handle) {
            material.params.time = time.elapsed_secs();
        }
    }
}
