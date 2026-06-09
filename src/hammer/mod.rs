use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub mod definition;
mod status;
mod systems;
mod trail_effect;
use crate::{
    game_play_set::GamePlaySet,
    hammer::status::BuffCounter,
    materials::MeteorMaterial,
    state::{InputMode, RunningState},
};
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
                    status::added_buff.in_set(GamePlaySet::Detection),
                    status::init_base_status.in_set(GamePlaySet::SetUp),
                    status::apply_buff.in_set(GamePlaySet::Logic),
                    apply_gravity_status.in_set(GamePlaySet::Logic),
                    apply_restitution_status.in_set(GamePlaySet::Logic),
                    (
                        (
                            handle_hammer_input_switch.run_if(in_state(InputMode::Switch)),
                            handle_hammer_input_hold.run_if(in_state(InputMode::Hold)),
                        )
                            .in_set(GamePlaySet::Input),
                        change_handle_direction.in_set(GamePlaySet::Detection),
                        update_hammer.in_set(GamePlaySet::Logic),
                        update_hammer_state_view.in_set(GamePlaySet::Rendering),
                    )
                        .chain(),
                    free_hammer.in_set(GamePlaySet::Detection),
                    pivot_texture,
                    trail_effect::attach_trail_effect,
                    fix_hammer_z,
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
