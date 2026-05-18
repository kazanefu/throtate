use crate::playing::MainCamera;

use super::*;
use bevy::prelude::*;

pub struct ChaserPlugin;

impl Plugin for ChaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (chase, chase_main_camera).in_set(UtilitySystemSet));
    }
}

#[derive(Component)]
pub struct Chaser(pub Option<Entity>);

fn chase(mut que: Query<(&mut Transform, &Chaser)>, target_que: Query<&GlobalTransform>) {
    for (mut transform, chaser) in &mut que {
        if chaser.0.is_none() {
            continue;
        }
        let target_translation = target_que
            .get(chaser.0.unwrap())
            .expect("target must have transform")
            .translation();
        transform.translation = target_translation;
    }
}

#[derive(Component)]
pub struct MainCameraChaser;

fn chase_main_camera(
    mut que: Query<&mut Transform, With<MainCameraChaser>>,
    target_que: Query<&Transform, (With<MainCamera>, Without<MainCameraChaser>)>,
) {
    let Ok(target) = target_que.single() else {
        return;
    };
    for mut transform in &mut que {
        (transform.translation.x, transform.translation.y) =
            (target.translation.x, target.translation.y);
    }
}
