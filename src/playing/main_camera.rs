use super::*;
use crate::state::GameState;

const FOLLOW_SPEED: f32 = 0.4;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, main_camera_follow_system);
    }
}

#[derive(Component)]
struct MainCamera {
    target: Entity,
}

pub fn main_camera_bundle(target: Entity) -> impl Bundle {
    (
        Camera2d,
        DespawnOnExit(GameState::Playing),
        MainCamera { target },
    )
}

fn main_camera_follow_system(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &MainCamera), Without<Player>>,
    target_query: Query<&Transform, With<Player>>,
) {
    for (mut camera_transform, target_entity) in &mut camera_query {
        let target_transform = target_query.get(target_entity.target).expect("main camera don't have target");
        camera_transform.translation = camera_transform.translation.lerp(target_transform.translation, FOLLOW_SPEED * time.delta_secs());
    }
}
