use crate::config::GameConfig;

use super::*;
pub struct AppExitPlugin;

impl Plugin for AppExitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, write_app_exit)
            .add_systems(OnEnter(GameState::Eixt), on_app_exit);
    }
}

fn write_app_exit(
    mut game_state: ResMut<NextState<GameState>>,
    config: Res<GameConfig>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if config.input.app_exit.just_pressed(&keys) {
        game_state.set(GameState::Eixt);
    }
}

fn on_app_exit(mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}
