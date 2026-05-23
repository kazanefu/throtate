use bevy::prelude::*;
mod breakable;
mod bullet;
mod checkpoint;
mod death;
mod death_vignette;
mod goal;
mod meteor;
mod space_background;
mod speedup;
mod turret;

pub use breakable::BreakableMaterial;
pub use bullet::BulletMaterial;
pub use checkpoint::CheckpointMaterial;
pub use death::DeathMaterial;
pub use goal::GoalMaterial;
pub use meteor::MeteorMaterial;
pub use speedup::SpeedupMaterial;
pub use turret::TurretMaterial;

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(death_vignette::DeathEffectPlugin)
            .add_plugins(meteor::MeteorMaterialPlugin)
            .add_plugins(space_background::SpaceBackGroundPlugin)
            .add_plugins(breakable::BreakableMaterialPlugin)
            .add_plugins(checkpoint::CheckpointMaterialPlugin)
            .add_plugins(death::DeathMaterialPlugin)
            .add_plugins(goal::GoalMaterialPlugin)
            .add_plugins(turret::TurretMaterialPlugin)
            .add_plugins(speedup::SpeedupMaterialPlugin)
            .add_plugins(bullet::BulletMaterialPlugin);
    }
}
