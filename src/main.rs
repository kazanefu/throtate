#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{prelude::*, window::WindowResolution};
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;

mod action_effect;
mod audio;
mod config;
mod course;
mod course_selection;
mod hammer;
mod materials;
mod playing;
mod result;
mod settings;
mod start;
mod state;
mod utils;

pub use utils::*;

fn main() {
    let settings = settings::get_settings();
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: settings.window.title.clone(),
            resolution: WindowResolution::new(settings.window.width, settings.window.height),
            mode: if settings.window.fullscreen {
                bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
            } else {
                bevy::window::WindowMode::Windowed
            },
            present_mode: if settings.window.vsync {
                bevy::window::PresentMode::AutoVsync
            } else {
                bevy::window::PresentMode::AutoNoVsync
            },
            ..default()
        }),
        ..default()
    }));
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");
    bevy::asset::embedded_asset!(app, "images/bluepivot.png");
    bevy::asset::embedded_asset!(app, "images/magentapivot.png");
    bevy::asset::embedded_asset!(app, "shaders/death_vignette.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/meteor.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/space_background.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/light_space_background.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/breakable.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/checkpoint.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/death.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/goal.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/turret.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/bullet.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/speedup.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/buff_spin_velocity.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/buff_spin_stiffness.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/buff_gravity_scale.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/buff_restitution.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/warp.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/death_breakable.wgsl");
    bevy::asset::embedded_asset!(app, "shaders/game_transition.wgsl");

    bevy::asset::embedded_asset!(app, "sounds/break.mp3");
    bevy::asset::embedded_asset!(app, "sounds/button_click.mp3");
    bevy::asset::embedded_asset!(app, "sounds/button_select.mp3");
    bevy::asset::embedded_asset!(app, "sounds/checkpoint.mp3");
    bevy::asset::embedded_asset!(app, "sounds/death_sound.mp3");
    bevy::asset::embedded_asset!(app, "sounds/goal.mp3");
    bevy::asset::embedded_asset!(app, "sounds/warp.mp3");
    bevy::asset::embedded_asset!(app, "sounds/wind.mp3");
    bevy::asset::embedded_asset!(app, "sounds/Beyond_the_Universe.mp3");
    bevy::asset::embedded_asset!(app, "sounds/プレリュード第3番「未知との邂逅」_2.mp3");
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(TimestepMode::Interpolated {
            dt: 1.0 / 120.0,
            time_scale: 1.0,
            substeps: 1,
        })
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(HanabiPlugin)
        .init_resource::<config::GameConfig>()
        .insert_resource(settings)
        .add_plugins(state::app_exit::AppExitPlugin)
        .add_plugins(utils::UtilityPlugin)
        .init_state::<state::GameState>()
        .init_state::<state::InputMode>()
        .init_state::<state::RunningState>()
        .add_plugins(course::CoursePlugin)
        .add_plugins(hammer::HammerPlugin)
        .add_plugins(start::StartPlugin)
        .add_plugins(playing::PlayingPlugin)
        .add_plugins(course_selection::CourseSelectionPlugin)
        .add_plugins(result::ResultPlugin)
        .add_plugins(action_effect::ActionEffectPlugin)
        .add_plugins(materials::CustomMaterialPlugin)
        .add_plugins(audio::AudioPlugin)
        .run();
}
