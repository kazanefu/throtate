use bevy::prelude::*;

pub struct GamePlaySetPlugin;

impl Plugin for GamePlaySetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GamePlaySet::SetUp,
                GamePlaySet::Input,
                GamePlaySet::Detection,
                GamePlaySet::Logic,
                GamePlaySet::Rendering,
                GamePlaySet::Audio,
            )
                .chain(),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GamePlaySet {
    SetUp,
    Input,
    // collision detection, HP monitoring, etc.
    Detection,
    // movement, update ui, animation, etc.
    Logic,
    // material update, visual effects, etc.
    Rendering,
    // sound effects, bgm, etc.
    Audio,
}
