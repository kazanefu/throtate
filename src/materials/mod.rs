use bevy::prelude::*;
mod death_vignette;
mod meteor;
mod space_background;
mod breakable;
mod checkpoint;

pub use meteor::MeteorMaterial;
pub use breakable::BreakableMaterial;
pub use checkpoint::CheckpointMaterial;

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(death_vignette::DeathEffectPlugin)
            .add_plugins(meteor::MeteorMaterialPlugin)
            .add_plugins(space_background::SpaceBackGroundPlugin)
            .add_plugins(breakable::BreakableMaterialPlugin)
            .add_plugins(checkpoint::CheckpointMaterialPlugin);
    }
}
