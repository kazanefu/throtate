use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct BreakableMaterialPlugin;

impl Plugin for BreakableMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BreakableMaterial>::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct BreakableMaterial {}

impl Material2d for BreakableMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/breakable.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}
