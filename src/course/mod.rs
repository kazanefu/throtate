use bevy::prelude::*;
use serde::Deserialize;
pub mod course_items;
mod load_course;
pub mod spawn;
pub use spawn::*;

use crate::state::GameState;

pub struct CoursePlugin;

impl Plugin for CoursePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(load_course::CourseLoadPlugin)
            .insert_resource(CourseListResource::default())
            .add_plugins(course_items::turret::TurretPlugin)
            .add_plugins(course_items::breakable_box::BreakableBoxPlugin)
            .add_plugins(course_items::speedup::SpeedUpPlugin)
            .add_message::<SpawnCourseMessage>() //init_courses_list_resource)
            .add_systems(Startup, setup_course_materials)
            .add_systems(Update, spawn_course_from_id);
    }
}

#[derive(Resource)]
pub struct CourseMaterials {
    pub breakable_mesh: Handle<Mesh>,
    pub breakable_material: Handle<crate::materials::BreakableMaterial>,
    pub checkpoint_mesh: Handle<Mesh>,
    pub checkpoint_material: Handle<crate::materials::CheckpointMaterial>,
}

fn setup_course_materials(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut breakable_materials: ResMut<Assets<crate::materials::BreakableMaterial>>,
    mut checkpoint_materials: ResMut<Assets<crate::materials::CheckpointMaterial>>,
    config: Res<crate::config::GameConfig>,
) {
    let box_size = config.course.one_box_size;
    commands.insert_resource(CourseMaterials {
        breakable_mesh: meshes.add(Rectangle::new(box_size, box_size)),
        breakable_material: breakable_materials.add(crate::materials::BreakableMaterial::default()),
        checkpoint_mesh: meshes.add(Rectangle::new(box_size, box_size)),
        checkpoint_material: checkpoint_materials.add(crate::materials::CheckpointMaterial::default()),
    });
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
    Ground {
        width: f32,
        height: f32,
    },
    Turret {
        interval: f32,
        rotation: f32,
        bullet_lifetime: Option<f32>,
    },
    Breakable {
        required_speed: f32,
    },
    Death,
    Checkpoint {
        priority: u32,
    },
    Goal,
    Text {
        sentence: String,
    },
    Dynamic {
        width: Option<f32>,
        height: Option<f32>,
        gravity_scale: Option<f32>,
        linear_damping: Option<f32>,
        angular_damping: Option<f32>,
        density_scale: Option<f32>,
        restitution_coefficient: Option<f32>,
    },
    SpeedUp {
        rate: f32,
    },
}

#[derive(Deserialize, Default)]
pub struct CourseList(pub Vec<CourseEntry>);

#[derive(Deserialize, Clone)]
pub struct CourseEntry {
    pub id: usize,
    pub name: String,
    pub path: String,
}
