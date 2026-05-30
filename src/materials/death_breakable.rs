use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

pub struct DeathBreakableMaterialPlugin;

impl Plugin for DeathBreakableMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DeathBreakableMaterial>::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct DeathBreakableMaterial {}

impl Material2d for DeathBreakableMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://throtate/shaders/death_breakable.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}
