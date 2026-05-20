use bevy::prelude::*;

pub const CLICK_PATH: &str = "embedded://throtate/sounds/button_click.mp3";
pub const SELECT_PATH: &str = "embedded://throtate/sounds/button_select.mp3";

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct ButtonSounds;
