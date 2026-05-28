use crate::playing::Player;
use crate::settings::Settings;
use crate::state::GameState;
use bevy::audio::Volume;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const WIND_PATH: &str = "embedded://throtate/sounds/wind.mp3";

#[derive(Resource)]
pub struct WindAsset {
    pub wind: Handle<AudioSource>,
}

#[derive(Component)]
pub struct WindSound;

pub struct WindSoundPlugin;

impl Plugin for WindSoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_wind_asset)
            .add_systems(OnEnter(GameState::Playing), start_wind_sound)
            .add_systems(OnExit(GameState::Playing), stop_wind_sound)
            .add_systems(
                Update,
                update_wind_volume.run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_wind_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(WindAsset {
        wind: asset_server.load(WIND_PATH),
    });
}

fn start_wind_sound(mut commands: Commands, wind_asset: Res<WindAsset>, _settings: Res<Settings>) {
    commands.spawn((
        WindSound,
        AudioPlayer(wind_asset.wind.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.0)), // 初期音量は0
    ));
}

fn stop_wind_sound(mut commands: Commands, wind_query: Query<Entity, With<WindSound>>) {
    for entity in &wind_query {
        commands.entity(entity).despawn();
    }
}

fn update_wind_volume(
    settings: Res<Settings>,
    player_query: Query<&Velocity, With<Player>>,
    mut wind_query: Query<&mut AudioSink, With<WindSound>>,
    time: Res<Time>,
    mut previous_velocity: Local<Option<Vec2>>,
    mut current_volume: Local<f32>,
) {
    let Ok(velocity) = player_query.single() else {
        return;
    };

    let Ok(mut sink) = wind_query.single_mut() else {
        return;
    };

    let current_vel = velocity.linvel;
    let speed = current_vel.length();

    // 加速度を計算
    let acceleration = if let Some(prev_vel) = *previous_velocity {
        let vel_change = current_vel - prev_vel;
        vel_change.length() / time.delta_secs()
    } else {
        0.0
    };

    *previous_velocity = Some(current_vel);

    // 速度と加速度に基づいて目標音量を計算
    // 速度の寄与: 速度が200以上で最大寄与0.6
    let speed_contribution = (speed / 200.0).min(1.0) * 0.1;

    // 加速度の寄与: 加速度が1000以上で最大寄与0.4（減らして破裂音を防ぐ）
    let accel_contribution = (acceleration / 1000.0).min(1.0) * 0.05;

    // 合計音量（最大1.0）にSE音量設定を適用
    let target_volume =
        (speed_contribution + accel_contribution).min(1.0) * settings.audio.se_volume;

    // 音量を滑らかに変化させる（スムージング）
    // 変化速度を制限して破裂音を防ぐ
    let smoothing_speed = 3.0; // 1秒あたりの最大変化量
    let max_change = smoothing_speed * time.delta_secs();
    let volume_diff = target_volume - *current_volume;
    let volume_change = volume_diff.clamp(-max_change, max_change);
    *current_volume += volume_change;
    *current_volume = current_volume.clamp(0.0, 1.0);

    let _ = sink.set_volume(Volume::Linear(*current_volume));
}
