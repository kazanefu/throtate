use crate::{JpFont, LifeTime, state::InputMode};
use bevy::prelude::*;

pub struct SwitchInputModePlugin;

impl Plugin for SwitchInputModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (switch_input_mode, spawn_input_mode_text).chain());
    }
}

pub fn switch_input_mode(
    mut next_input_mode: ResMut<NextState<InputMode>>,
    now_input_mode: Res<State<InputMode>>,
    keys: Res<ButtonInput<KeyCode>>,
    config: Res<crate::config::GameConfig>,
) {
    if config.input.switch_input_mode.just_released(&keys) {
        next_input_mode.set(match now_input_mode.get() {
            InputMode::Hold => InputMode::Switch,
            InputMode::Switch => InputMode::Hold,
        });
    }
}

pub fn spawn_input_mode_text(
    mut commands: Commands,
    font: Res<JpFont>,
    input_mode: Res<State<InputMode>>,
) {
    if !input_mode.is_changed() {
        return;
    }
    let text = match input_mode.get() {
        InputMode::Hold => "Hold",
        InputMode::Switch => "Switch",
    };
    commands.spawn((
        Text::new(text),
        TextFont {
            font: font.font.clone(),
            font_size: 40.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
        TextColor::WHITE,
        LifeTime::new(2.0),
    ));
}
