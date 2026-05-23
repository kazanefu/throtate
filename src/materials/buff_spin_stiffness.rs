use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct BuffSpinStiffnessMaterialPlugin;

impl Plugin for BuffSpinStiffnessMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BuffSpinStiffnessMaterial>::default())
            .add_systems(Update, update_buff_spin_stiffness_material);
    }
}

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct BuffSpinStiffnessUniform {
    pub time: f32,
    pub color: Vec4,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct BuffSpinStiffnessMaterial {
    #[uniform(0)]
    pub params: BuffSpinStiffnessUniform,
}

impl BuffSpinStiffnessMaterial {
    pub fn new(color: Color) -> Self {
        Self {
            params: BuffSpinStiffnessUniform {
                time: 0.0,
                color: color.to_linear().to_vec4(),
            },
        }
    }
}

impl Material2d for BuffSpinStiffnessMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/buff_spin_stiffness.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

fn update_buff_spin_stiffness_material(
    time: Res<Time>,
    mut materials: ResMut<Assets<BuffSpinStiffnessMaterial>>,
    query: Query<&MeshMaterial2d<BuffSpinStiffnessMaterial>>,
) {
    for handle in &query {
        if let Some(material) = materials.get_mut(handle) {
            material.params.time = time.elapsed_secs();
        }
    }
}
