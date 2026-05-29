pub mod bgm;
pub mod break_sound;
pub mod button;
pub mod checkpoint;
pub mod death;
pub mod goal;
pub mod warp;
pub mod wind;

use crate::keyboard_button::KeyboardHovered;
use crate::settings::Settings;
use bevy::audio::Volume;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct AudioAssets {
    pub death: Handle<AudioSource>,
    pub break_sound: Handle<AudioSource>,
    pub button_click: Handle<AudioSource>,
    pub button_select: Handle<AudioSource>,
    pub checkpoint: Handle<AudioSource>,
    pub goal: Handle<AudioSource>,
    pub warp: Handle<AudioSource>,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bgm::BgmPlugin)
            .add_plugins(wind::WindSoundPlugin)
            .add_systems(Startup, setup_audio_assets)
            .add_systems(Update, play_button_sounds);
    }
}

fn setup_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        death: asset_server.load(death::PATH),
        break_sound: asset_server.load(break_sound::PATH),
        button_click: asset_server.load(button::CLICK_PATH),
        button_select: asset_server.load(button::SELECT_PATH),
        checkpoint: asset_server.load(checkpoint::PATH),
        goal: asset_server.load(goal::PATH),
        warp: asset_server.load(warp::PATH),
    });
}

type ButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static Interaction,
        Option<&'static mut KeyboardHovered>,
    ),
    (With<button::ButtonSounds>, Changed<Interaction>),
>;

fn play_button_sounds(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    settings: Res<Settings>,
    mut query: ButtonInteractionQuery,
    all_buttons: Query<(), With<button::ButtonSounds>>,
    mut previous_states: Local<HashMap<Entity, Interaction>>,
) {
    for (entity, interaction, keyboard_hovered) in &mut query {
        let prev = previous_states
            .get(&entity)
            .copied()
            .unwrap_or(Interaction::None);
        let mut interaction_recoad = Interaction::None;
        match (*interaction, prev) {
            (Interaction::Pressed, _) => {
                commands.spawn((
                    AudioPlayer(audio_assets.button_click.clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.audio.se_volume)),
                ));
                interaction_recoad = Interaction::Pressed;
            }
            (Interaction::Hovered, Interaction::None) => {
                commands.spawn((
                    AudioPlayer(audio_assets.button_select.clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.audio.se_volume)),
                ));
                interaction_recoad = Interaction::Hovered;
            }
            (Interaction::Hovered, _) => {
                interaction_recoad = Interaction::Hovered;
            }
            _ => {}
        }
        if let Some(mut hovered) = keyboard_hovered
            && hovered.0
            && prev == Interaction::None
        {
            commands.spawn((
                AudioPlayer(audio_assets.button_select.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(settings.audio.se_volume)),
            ));
            interaction_recoad = Interaction::Hovered;
            hovered.0 = false;
        }
        previous_states.insert(entity, interaction_recoad);
    }

    // Clean up despawned buttons to prevent memory leak
    previous_states.retain(|entity, _| all_buttons.contains(*entity));
}
