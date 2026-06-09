use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum InputMode {
    #[default]
    Hold,
    Switch,
}
