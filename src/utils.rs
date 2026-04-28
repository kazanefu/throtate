use crate::state::RunningState;
use bevy::prelude::*;
pub const FONT_PATH: &str = "embedded://throtate/fonts/NotoSansJP-Bold.ttf";

pub struct UtilityPlugin;

impl Plugin for UtilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (tick_interval, update_stopwatch, update_despawn_timer, despawn_timeout_entity).run_if(in_state(RunningState::Running)),
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
pub struct DespawnWithTime(pub f32);

pub fn update_despawn_timer(time: Res<Time>, mut despawn_timer_query: Query<&mut DespawnWithTime>) {
    for mut despawn_timer in &mut despawn_timer_query {
        despawn_timer.0 -= time.delta_secs();
    }
}

pub fn despawn_timeout_entity(mut commands: Commands, query: Query<(Entity, &DespawnWithTime)>) {
    for (entity, timer) in &query {
        if timer.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
