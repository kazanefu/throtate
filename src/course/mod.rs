use bevy::prelude::*;
use serde::Deserialize;
mod load_course;
use load_course::*;
pub mod course_items;
pub mod spawn;
pub use spawn::*;

use crate::state::GameState;

pub struct CoursePlugin;

impl Plugin for CoursePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CourseListResource::default())
            .init_asset::<asset_load::RonText>()
            .register_asset_loader(asset_load::RonTextLoader)
            .add_plugins(course_items::turret::TurretPlugin)
            .add_plugins(course_items::breakable_box::BreakableBoxPlugin)
            .add_message::<SpawnCourseMessage>()
            .init_resource::<asset_load::CourseLoadState>()
            .add_systems(OnEnter(GameState::Start), asset_load::start_load_courses)//init_courses_list_resource)
            .add_systems(Update, (spawn_course_from_id, asset_load::resolve_courses));
    }
}

#[derive(Resource, Default)]
pub struct CourseListResource(pub Vec<(CourseEntry, Course)>);

#[derive(Component)]
pub struct CourseID {
    #[allow(unused)]
    id: usize,
}

impl CourseID {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

#[derive(Deserialize, Default)]
pub struct Course {
    pub entities: Vec<EntityData>,
}

#[derive(Deserialize)]
pub struct EntityData {
    pub x: f32,
    pub y: f32,
    pub kind: EntityKind,
}

#[derive(Deserialize)]
pub enum EntityKind {
    Ground { width: f32, height: f32 },
    Turret { interval: f32, rotation: f32 },
    Breakable { required_speed: f32 },
    Death,
    Checkpoint { priority: u32 },
    Goal,
    Text { sentence: String },
}

#[derive(Deserialize, Default)]
pub struct CourseList(pub Vec<CourseEntry>);

#[derive(Deserialize, Clone)]
pub struct CourseEntry {
    pub id: usize,
    pub name: String,
    pub path: String,
}
