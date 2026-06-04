use bevy::prelude::*;

pub struct LookAtPlugin;

impl Plugin for LookAtPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, look_at_system);
    }
}

#[derive(Component, Default)]
pub struct LookAt {
    pub target: Option<Entity>,
}
#[derive(Component)]
pub struct LookAtTarget;

pub fn look_at_system(
    mut que: Query<(&GlobalTransform, &mut Transform, &LookAt)>,
    target_que: Query<&GlobalTransform, With<LookAtTarget>>,
) {
    for (global_transform, mut transform, look_at) in &mut que {
        let Some(target_entity) = look_at.target else {
            continue;
        };
        let Ok(target_transform) = target_que.get(target_entity) else {
            continue;
        };

        let self_pos = global_transform.translation().truncate();
        let target_pos = target_transform.translation().truncate();

        let dir = target_pos - self_pos;

        if dir.length_squared() > 0.0 {
            let angle = dir.y.atan2(dir.x);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
