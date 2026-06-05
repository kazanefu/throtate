use bevy::prelude::*;
pub mod app_exit;
mod input_mode;

pub use input_mode::InputMode;

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
