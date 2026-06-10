use bevy::prelude::*;
pub mod app_exit;
mod input_mode;

pub use input_mode::InputMode;

use crate::{config::GameConfig, game_play_set::GamePlaySet};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum GameState {
    #[default]
    Loading,
    Start,
    CourseSelection,
    Playing,
    Result,
    Eixt,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum RunningState {
    #[default]
    Running,
    #[allow(unused)]
    Paused,
}

pub struct RunningStatePlugin;

impl Plugin for RunningStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RunningState>()
            .configure_sets(
                Update,
                bevy_rapier2d::prelude::PhysicsSet::StepSimulation
                    .run_if(in_state(RunningState::Running)),
            )
            .add_message::<RunningStateControlMsg>()
            .add_systems(
                Update,
                (
                    pause_unpause_input.in_set(GamePlaySet::Input),
                    set_running_state.in_set(GamePlaySet::Logic),
                ),
            );
    }
}

#[derive(Message)]
pub enum RunningStateControlMsg {
    Pause,
    Resume,
}

fn set_running_state(
    mut time: ResMut<Time<Virtual>>,
    mut msg: MessageReader<RunningStateControlMsg>,
    mut running_state: ResMut<NextState<RunningState>>,
) {
    for next_state in msg.read() {
        match next_state {
            RunningStateControlMsg::Pause => {
                time.pause();
                running_state.set(RunningState::Paused);
            }
            RunningStateControlMsg::Resume => {
                time.unpause();
                running_state.set(RunningState::Running);
            }
        }
    }
}

fn pause_unpause_input(
    keys: Res<ButtonInput<KeyCode>>,
    running_state: Res<State<RunningState>>,
    mut msg: MessageWriter<RunningStateControlMsg>,
    config: Res<GameConfig>,
) {
    if config.input.pause_unpause.just_released(&keys) {
        msg.write(match **running_state {
            RunningState::Paused => RunningStateControlMsg::Resume,
            RunningState::Running => RunningStateControlMsg::Pause,
        });
    }
}
