use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct BuffGravityScaleMaterialPlugin;

impl Plugin for BuffGravityScaleMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BuffGravityScaleMaterial>::default())
            .add_systems(Update, update_buff_gravity_scale_material);
    }
}

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct BuffGravityScaleUniform {
    pub time: f32,
    pub color: Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct BuffGravityScaleMaterial {
    #[uniform(0)]
    pub params: BuffGravityScaleUniform,
}

impl BuffGravityScaleMaterial {
    pub fn new(color: Color) -> Self {
        Self {
            params: BuffGravityScaleUniform {
                time: 0.0,
                color: color.to_linear().to_vec4(),
            },
        }
    }
}

impl Material2d for BuffGravityScaleMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/buff_gravity_scale.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

fn update_buff_gravity_scale_material(
    time: Res<Time>,
    mut materials: ResMut<Assets<BuffGravityScaleMaterial>>,
    query: Query<&MeshMaterial2d<BuffGravityScaleMaterial>>,
) {
    for handle in &query {
        if let Some(material) = materials.get_mut(handle) {
            material.params.time = time.elapsed_secs();
        }
    }
}
