use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub mod definition;
mod status;
mod systems;
mod trail_effect;
use crate::{hammer::status::BuffCounter, materials::MeteorMaterial, state::RunningState};
use definition::*;
#[allow(unused)]
pub use status::{Buff, BuffStatusChannel, BuffType, FinalStatus};
use systems::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HammerSystemSet;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<HammerActionMessage>()
            .add_message::<ChangeHandleDirection>()
            .add_message::<HammerFreeMessage>()
            .insert_resource(BuffCounter::default())
            .add_systems(
                Startup,
                (load_pivot_texture, trail_effect::setup_trail_effect),
            )
            .configure_sets(
                Update,
                HammerSystemSet.run_if(in_state(RunningState::Running)),
            )
            .add_systems(
                Update,
                (
                    handle_hammer_input,
                    status::added_buff,
                    status::init_base_status,
                    status::apply_buff,
                    apply_gravity_status,
                    apply_restitution_status,
                    update_hammer,
                    free_hammer,
                    change_handle_direction,
                    pivot_texture,
                    trail_effect::attach_trail_effect,
                    fix_hammer_z,
                    update_hammer_state_view,
                )
                    .in_set(HammerSystemSet),
            );
    }
}

#[allow(unused)]
pub fn spawn_hammer<'a>(
    commands: &'a mut Commands,
    translate: Vec2,
    mut meshes: &mut Assets<Mesh>,
    mut meteor_materials: &mut Assets<MeteorMaterial>,
    config: &crate::config::GameConfig,
) -> EntityCommands<'a> {
    let pivot = commands
        .spawn((
            RigidBody::Fixed,
            Transform::from_xyz(translate.x - 1.0, translate.y - 1.0, 10.0),
            Pivot,
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
        ))
        .id();
    commands.spawn(hammer_bundle(
        meshes,
        meteor_materials,
        pivot,
        translate,
        &config.hammer,
    ))
}

fn load_pivot_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PivotTextures {
        blue: asset_server.load("embedded://throtate/images/bluepivot.png"),
        magenta: asset_server.load("embedded://throtate/images/magentapivot.png"),
    });
}
