use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod definition;
mod systems;
use definition::*;
use systems::*;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<HammerActionMessage>().add_message::<ChangeHandleDirection>().add_systems(
            Update,
            (handle_hammer_input, update_hammer, change_handle_direction),
        );
    }
}



#[allow(unused)]
pub fn spawn_hammer<'a>(commands: &'a mut Commands, translate: Vec2) -> EntityCommands<'a> {
    let pivot = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(translate.x - 1.0, translate.y - 1.0, 0.0),
            Pivot,
        ))
        .id();
    commands.spawn(hammer_bundle(pivot, translate))
}


