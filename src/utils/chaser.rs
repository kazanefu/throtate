use super::*;
use bevy::prelude::*;

pub struct ChaserPlugin;

impl Plugin for ChaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, chase.in_set(UtilitySystemSet));
    }
}

#[derive(Component)]
pub struct Chaser(pub Entity);

fn chase(mut que: Query<(&mut Transform, &Chaser)>, target_que: Query<&GlobalTransform>) {
    for (mut transform, chaser) in &mut que {
        let target_translation = target_que
            .get(chaser.0)
            .expect("target must have transform")
            .translation();
        transform.translation = target_translation;
    }
}
