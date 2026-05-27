use crate::config::GameConfig;

use super::*;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RespawnMessage>()
            .add_systems(Update, handle_respawn.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Message)]
pub struct RespawnMessage;

fn handle_respawn(
    keys: Res<ButtonInput<KeyCode>>,
    mut message: MessageWriter<RespawnMessage>,
    input_setting: Res<GameConfig>,
) {
    if input_setting.input.respawn.just_pressed(&keys) {
        message.write(RespawnMessage);
    }
}
