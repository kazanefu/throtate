use bevy::prelude::*;
mod breakable;
mod buff_gravity_scale;
mod buff_restitution;
mod buff_spin_stiffness;
mod buff_spin_velocity;
mod bullet;
mod checkpoint;
mod death;
mod death_vignette;
mod goal;
mod meteor;
mod space_background;
mod speedup;
mod turret;
mod warp;

pub use breakable::BreakableMaterial;
pub use buff_gravity_scale::BuffGravityScaleMaterial;
pub use buff_restitution::BuffRestitutionMaterial;
pub use buff_spin_stiffness::BuffSpinStiffnessMaterial;
pub use buff_spin_velocity::BuffSpinVelocityMaterial;
pub use bullet::BulletMaterial;
pub use checkpoint::CheckpointMaterial;
pub use death::DeathMaterial;
pub use goal::GoalMaterial;
pub use meteor::MeteorMaterial;
pub use speedup::SpeedupMaterial;
pub use turret::TurretMaterial;
pub use warp::WarpMaterial;

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(death_vignette::DeathEffectPlugin)
            .add_plugins(meteor::MeteorMaterialPlugin)
            .add_plugins(space_background::SpaceBackGroundPlugin)
            .add_plugins(breakable::BreakableMaterialPlugin)
            .add_plugins(buff_gravity_scale::BuffGravityScaleMaterialPlugin)
            .add_plugins(buff_restitution::BuffRestitutionMaterialPlugin)
            .add_plugins(buff_spin_stiffness::BuffSpinStiffnessMaterialPlugin)
            .add_plugins(buff_spin_velocity::BuffSpinVelocityMaterialPlugin)
            .add_plugins(checkpoint::CheckpointMaterialPlugin)
            .add_plugins(death::DeathMaterialPlugin)
            .add_plugins(goal::GoalMaterialPlugin)
            .add_plugins(turret::TurretMaterialPlugin)
            .add_plugins(speedup::SpeedupMaterialPlugin)
            .add_plugins(bullet::BulletMaterialPlugin)
            .add_plugins(warp::WarpMaterialPlugin);
    }
}
