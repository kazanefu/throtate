use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct SpeedupMaterialPlugin;

impl Plugin for SpeedupMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<SpeedupMaterial>::default())
            .add_systems(Update, update_speedup_material);
    }
}

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct SpeedupUniform {
    pub time: f32,
    pub base_color: Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct SpeedupMaterial {
    #[uniform(0)]
    pub params: SpeedupUniform,
}
impl SpeedupMaterial {
    pub fn new(base_color: Color) -> Self {
        Self {
            params: SpeedupUniform {
                time: 0.0,
                base_color: base_color.to_linear().to_vec4(),
            },
        }
    }
}

impl Material2d for SpeedupMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/speedup.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

fn update_speedup_material(
    time: Res<Time>,
    mut materials: ResMut<Assets<SpeedupMaterial>>,
    query: Query<&MeshMaterial2d<SpeedupMaterial>>,
) {
    for handle in &query {
        if let Some(material) = materials.get_mut(handle) {
            material.params.time = time.elapsed_secs();
        }
    }
}
