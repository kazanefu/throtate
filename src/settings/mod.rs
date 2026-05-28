use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
mod load;

pub use load::get_settings;

#[derive(Resource, Default, Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub window: WindowSettings,
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub fullscreen: bool,
    pub vsync: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "throtate".into(),
            fullscreen: false,
            vsync: true,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct GraphicsSettings {
    pub light_background: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AudioSettings {
    pub bgm_volume: f32,
    pub se_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            bgm_volume: 0.5,
            se_volume: 0.7,
        }
    }
}
