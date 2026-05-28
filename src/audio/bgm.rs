use crate::settings::Settings;
use crate::state::GameState;
use bevy::audio::Volume;
use bevy::prelude::*;

pub const PRELUDE_PATH: &str = "embedded://throtate/sounds/プレリュード第3番「未知との邂逅」_2.mp3";
pub const BEYOND_THE_UNIVERSE_PATH: &str = "embedded://throtate/sounds/Beyond_the_Universe.mp3";

#[derive(Resource)]
pub struct BgmAssets {
    pub prelude: Handle<AudioSource>,
    pub beyond_the_universe: Handle<AudioSource>,
}

#[derive(Component)]
pub struct BgmPlayer;

#[derive(Component)]
pub enum BgmType {
    Prelude,
    BeyondTheUniverse,
}

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bgm_assets)
            .add_systems(OnEnter(GameState::Start), play_prelude_bgm)
            .add_systems(OnEnter(GameState::CourseSelection), play_prelude_bgm)
            .add_systems(OnEnter(GameState::Result), play_prelude_bgm)
            .add_systems(OnEnter(GameState::Playing), play_beyond_the_universe_bgm)
            .add_systems(Update, update_bgm_volume);
    }
}

fn setup_bgm_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BgmAssets {
        prelude: asset_server.load(PRELUDE_PATH),
        beyond_the_universe: asset_server.load(BEYOND_THE_UNIVERSE_PATH),
    });
}

fn play_prelude_bgm(
    mut commands: Commands,
    bgm_assets: Res<BgmAssets>,
    settings: Res<Settings>,
    existing_bgm: Query<Entity, With<BgmPlayer>>,
) {
    // 既存のBGMを停止
    for entity in &existing_bgm {
        commands.entity(entity).despawn();
    }

    // プレリュード再生
    commands.spawn((
        BgmPlayer,
        BgmType::Prelude,
        AudioPlayer(bgm_assets.prelude.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(settings.audio.bgm_volume)),
    ));
}

fn play_beyond_the_universe_bgm(
    mut commands: Commands,
    bgm_assets: Res<BgmAssets>,
    settings: Res<Settings>,
    existing_bgm: Query<Entity, With<BgmPlayer>>,
) {
    // 既存のBGMを停止
    for entity in &existing_bgm {
        commands.entity(entity).despawn();
    }

    // Beyond the Universeを再生
    commands.spawn((
        BgmPlayer,
        BgmType::BeyondTheUniverse,
        AudioPlayer(bgm_assets.beyond_the_universe.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(settings.audio.bgm_volume)),
    ));
}

fn update_bgm_volume(
    settings: Res<Settings>,
    mut bgm_query: Query<&mut AudioSink, With<BgmPlayer>>,
) {
    if !settings.is_changed() {
        return;
    }

    for mut sink in &mut bgm_query {
        sink.set_volume(Volume::Linear(settings.audio.bgm_volume));
    }
}
