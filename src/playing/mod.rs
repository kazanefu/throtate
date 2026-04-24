use bevy::prelude::*;
mod startup;
mod player;
mod main_camera;
pub use player::*;
pub use main_camera::*;
pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(startup::PlayingStartupPlugin).add_plugins(main_camera::MainCameraPlugin);
    }
}
