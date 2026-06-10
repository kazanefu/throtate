pub mod button;
pub mod chaser;
pub mod collision;
pub mod keyboard_button;
pub mod look_at;
pub mod scroll_ui;
pub mod switch_input_mode;
use crate::state::RunningState;
use bevy::prelude::*;
pub const FONT_PATH: &str = "embedded://throtate/fonts/NotoSansJP-Bold.ttf";

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UtilitySystemSet;

pub struct UtilityPlugin;

impl Plugin for UtilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_jp_font)
            .configure_sets(
                Update,
                UtilitySystemSet.run_if(in_state(RunningState::Running)),
            )
            .add_plugins(button::ButtonPlugin)
            .add_plugins(chaser::ChaserPlugin)
            .add_plugins(keyboard_button::KeyboardButtonPlugin)
            .add_plugins(look_at::LookAtPlugin)
            .add_plugins(switch_input_mode::SwitchInputModePlugin)
            .add_systems(
                Update,
                (
                    tick_interval,
                    update_stopwatch,
                    start_life_time,
                    despawn_life_end,
                )
                    .in_set(UtilitySystemSet),
            );
    }
}

#[derive(Component, Clone, Copy)]
pub struct Interval {
    pub time: f32,
    pub interval: f32,
}

impl Interval {
    pub fn tick(&mut self, delta_time: f32) {
        self.time += delta_time;
    }
    pub fn reset(&mut self) {
        self.time = 0.0;
    }
    pub fn is_ready(&self) -> bool {
        self.time >= self.interval
    }
}

fn tick_interval(time: Res<Time>, query: Query<&mut Interval>) {
    for mut interval in query {
        interval.tick(time.delta_secs());
    }
}

#[derive(Component, Default)]
pub struct StopWatch {
    time: f32,
    is_running: bool,
}

impl StopWatch {
    pub fn new(run: bool) -> Self {
        Self {
            time: 0.0,
            is_running: run,
        }
    }
    pub fn now(&self) -> f32 {
        self.time
    }
    pub fn start(&mut self) {
        self.is_running = true;
    }
    pub fn pause(&mut self) {
        self.is_running = false;
    }
    pub fn reset(&mut self) {
        self.time = 0.0;
    }
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

pub fn update_stopwatch(time: Res<Time>, mut stopwatch_query: Query<&mut StopWatch>) {
    for mut stopwatch in &mut stopwatch_query {
        if stopwatch.is_running() {
            stopwatch.time += time.delta_secs();
        }
    }
}

#[derive(Component)]
pub struct LifeTime {
    life_time: f32,
    start_time: Option<f32>,
}
impl LifeTime {
    pub fn new(life_time: f32) -> Self {
        Self {
            life_time,
            start_time: None,
        }
    }
}

fn start_life_time(time: Res<Time>, mut life_time_que: Query<&mut LifeTime, Added<LifeTime>>) {
    for mut life_time in &mut life_time_que {
        life_time.start_time = Some(time.elapsed_secs());
    }
}
fn despawn_life_end(
    time: Res<Time>,
    life_time_que: Query<(Entity, &LifeTime)>,
    mut commands: Commands,
) {
    let now = time.elapsed_secs();
    for (entity, life_time) in &life_time_que {
        if let Some(start) = life_time.start_time
            && now - start >= life_time.life_time
        {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Resource)]
pub struct JpFont {
    font: Handle<Font>,
}
impl JpFont {
    pub fn get(&self) -> &Handle<Font> {
        &self.font
    }
}

fn load_jp_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT_PATH);
    commands.insert_resource(JpFont { font });
}
